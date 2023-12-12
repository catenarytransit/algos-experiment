
use actix_web::middleware::DefaultHeaders;

mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Deserialize, PostgresMapper, Serialize, Debug, Clone)]
    #[pg_mapper(table = "timetable")]
    pub struct TimeTable {
        pub direction: String,
        pub id: String,
        pub route: String,
        pub stop: String,
        pub service: String,
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

    pub async fn index(client: &Client, id: String, route: String, stop: String, service: String, direction: String) -> Result<Vec<TimeTable>, MyError> {
        let stmt = "SELECT * FROM timetable WHERE id LIKE $1 AND route LIKE $2 AND stop LIKE $3 AND service LIKE $4 AND direction LIKE $5";

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
        let id = match qs.get("onestop_id") {
            Some(id) => id.to_string(),
            None => "%".to_string(),
        };
        let route = match qs.get("route") {
            Some(route) => route.to_string(),
            None => "%".to_string(),
        };
        let stop = match qs.get("stop") {
            Some(stop) => stop.to_string(),
            None => "%".to_string(),
        };
        let service = match qs.get("service") {
            Some(service) => service.to_string(),
            None => "%".to_string(),
        };
        let direction = match qs.get("direction") {
            Some(direction) => direction.to_string(),
            None => "%".to_string(),
        };
        let re = Regex::new("%+").unwrap();
        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;
        let timetable = db::index(&client, id, route, stop, service, direction).await?;
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
        App::new()
        .wrap(actix_block_ai_crawling::BlockAi)
        .wrap(
            DefaultHeaders::new()
        .add((
            "Access-Control-Allow-Origin",
            "*",
        ))
        )
        .app_data(web::Data::new(pool.clone())).service(
            web::resource("/")
                .route(web::get().to(index)),
        )
    })
    .bind("127.0.0.1:8080")?
    .run();
    println!("Server running at http://127.0.0.1:8080/");

    server.await
}