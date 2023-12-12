mod models {
    use serde::{Deserialize, Serialize};
    use tokio_pg_mapper_derive::PostgresMapper;

    #[derive(Deserialize, PostgresMapper, Serialize)]
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
        let stmt = "SELECT * FROM timetable WHERE id = $1";

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
    use crate::{db, errors::MyError, models::TimeTable};

    pub async fn index(db_pool: web::Data<Pool>, req: HttpRequest) -> Result<HttpResponse, Error> {
        let qs = QString::from(req.query_string());
        let onestop_id = qs.get("onestop_id").unwrap_or_else(|| "f-9q5-metro~losangeles~rail");
        let route = qs.get("route").unwrap_or_else(|| "801");   
        let stop = qs.get("stop").unwrap_or_else(|| "80105");
        let service = qs.get("service").unwrap_or_else(|| "RDEC23-801-2_Saturday-90");
        let direction = qs.get("direction").unwrap_or_else(|| "1");
        let formatted_args = format!("{}-{}-{}-{}-{}", onestop_id, route, stop, service, direction);
        let client: Client = db_pool.get().await.map_err(MyError::PoolError)?;

        let timetable = db::index(&client, formatted_args).await?;

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