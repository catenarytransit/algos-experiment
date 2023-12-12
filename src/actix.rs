mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Deserialize, PostgresMapper, Serialize, Debug, Clone)]
    #[pg_mapper(table = "timetable")]
    pub struct TimeTable {
        pub id: String,
        pub time: String,
    }
}

mod errors {
    use actix_web::{HttpResponse, ResponseError};
    use deadpool_postgres::PoolError;
    use derive_more::{Display, From};
    use tokio_pg_mapper::Error as PGMError;
    use tokio_postgres::error::Error as PGError;

    #[derive(Display, From, Debug)]
    pub enum MyError {
        NotFound,
        PGError(PGError),
        PGMError(PGMError),
        PoolError(PoolError),
    }
    impl std::error::Error for MyError {}

    impl ResponseError for MyError {
        fn error_response(&self) -> HttpResponse {
            match *self {
                MyError::NotFound => HttpResponse::NotFound().finish(),
                MyError::PoolError(ref err) => {
                    HttpResponse::InternalServerError().body(err.to_string())
                }
                _ => HttpResponse::InternalServerError().finish(),
            }
        }
    }
}

mod db {
    use deadpool_postgres::Client;
    use tokio_pg_mapper::FromTokioPostgresRow;

    use crate::{errors::MyError, models::TimeTable};

    pub async fn index(client: &Client, id: String) -> Result<Vec<TimeTable>, MyError> {
        let stmt = "SELECT * FROM timetable WHERE id LIKE $1";

        let results = client
            .query(stmt, &[&id])
            .await?
            .iter()
            .map(|row| TimeTable::from_row_ref(row).unwrap())
            .collect::<Vec<TimeTable>>();

        Ok(results)
    }
}

mod handlers {
    use actix_web::{web, Error, HttpResponse, HttpRequest};
    use deadpool_postgres::{Client, Pool};
    use qstring::QString;
    use regex::Regex;
    use crate::{db, errors::MyError};

    pub async fn index(db_pool: web::Data<Pool>, req: HttpRequest) -> Result<HttpResponse, Error> {
        let qs = QString::from(req.query_string());
        println!("{:?}", qs);
        let mut formatted_args = String::new();
        match qs.get("onestop_id") {
            Some(id) => formatted_args.push_str(id),
            None => formatted_args.push_str("%"),
        };
        match qs.get("route") {
            Some(route) => formatted_args.push_str(format!("-{}", route).as_str()),
            None => formatted_args.push_str("%"),
        };
        match qs.get("stop") {
            Some(stop) => formatted_args.push_str(format!("-{}", stop).as_str()),
            None => formatted_args.push_str("%"),
        };
        match qs.get("service") {
            Some(service) => formatted_args.push_str(format!("-{}", service).as_str()),
            None => formatted_args.push_str("%"),
        };
        match qs.get("direction") {
            Some(direction) => formatted_args.push_str(format!("-{}", direction).as_str()),
            None => formatted_args.push_str("%"),
        };
        let re = Regex::new("%+").unwrap();
        let formatted_args = re.replace_all(formatted_args.as_str(), "%").to_string();
        println!("{}", formatted_args);
        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
        let timetable = db::index(&client, formatted_args).await?;
        //println!("{:#?}", timetable.clone());
        let last = timetable.last().unwrap().time.clone();
        let last_string = last.to_string();
        Ok(HttpResponse::Ok().json(timetable))
    }
}

use actix_web::{web, App, HttpServer};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use tokio_postgres::NoTls;
use handlers::index;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let mut pg_config = tokio_postgres::Config::new();
    pg_config.host_path("/run/postgresql");
    pg_config.host_path("/tmp");
    pg_config.user("lolpro11");
    pg_config.dbname("catenary");
    let mgr_config = ManagerConfig {
        recycling_method: RecyclingMethod::Fast
    };
    let mgr = Manager::from_config(pg_config, NoTls, mgr_config);
    let pool = Pool::builder(mgr).max_size(16).build().unwrap();

    let server = HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::resource("/")
                .route(web::get().to(index)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run();
    println!("Server running at http://127.0.0.1:8080/");

    server.await
}