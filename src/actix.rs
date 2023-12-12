use actix_web::{web, App, HttpServer, Responder, HttpResponse, HttpRequest, middleware::DefaultHeaders};
use qstring::QString;
use tokio_postgres::NoTls;
extern crate qstring;

async fn index(req: HttpRequest) -> impl Responder {
    println!("Ping");
    let qs = QString::from(req.query_string());

    // Handling the Result from tokio_postgres::connect
    let result = tokio_postgres::connect("postgresql://lolpro11:lolpro11@localhost/catenary", NoTls).await;
    let (client, _connection) = match result {
        Ok((client, connection)) => (client, connection),
        Err(err) => {
            eprintln!("Failed to connect to the database: {}", err);
            return HttpResponse::InternalServerError().finish();
        }
    };
    println!("connected to PG");
    let onestop_id = qs.get("onestop_id").unwrap_or_else(|| "f-9q5-metro~losangeles~rail");
    let route = qs.get("route").unwrap_or_else(|| "801");   
    let stop = qs.get("stop").unwrap_or_else(|| "80105");
    let service = qs.get("service").unwrap_or_else(|| "RDEC23-801-2_Saturday-90");
    let direction = qs.get("direction").unwrap_or_else(|| "1");
    let formatted_args = format!("{}-{}-{}-{}-{}", onestop_id, route, stop, service, direction);
    let query = "SELECT * FROM timetable WHERE id = $1";
    println!("Executing query: {} with params: {:?}", query, formatted_args);
    let rows = client.query(query,  &[&formatted_args]).await;
    println!("{:?}", rows);

    HttpResponse::Ok().body(format!("{:?}", rows.unwrap()))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let builder = HttpServer::new(|| {
        App::new().route("/", web::get().to(index))
    }).workers(4);
    let _ = builder.bind("127.0.0.1:8080").unwrap().run().await;
    Ok(())
}
