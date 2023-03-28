use std::error::Error;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use postgres::{Client, NoTls};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct AlertRequest {
    pub id: i32,
    pub user_id: String,
    pub asset_id: String,
    pub above: bool,
    pub price_alert: String,
    pub notified: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AddAlertRequest {
    pub user_id: String,
    pub asset_id: String,
    pub above: bool,
    pub price_alert: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteAlertRequest {
    pub id: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FetchUserAlertRequest {
    pub user_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAlertRequest {
    pub id: i32,
    pub user_id: String,
    pub asset_id: String,
    pub above: bool,
    pub price_alert: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetPrice {
    id: i32,
    asset_id: String,
    price: String,
}

#[post("/add_alert_request")]
async fn add_alert_request(alert_request: web::Json<AddAlertRequest>) -> impl Responder {
    let mut alert_request = alert_request.into_inner();

    // Validate the request body
    if alert_request.user_id.is_empty() {
        return HttpResponse::BadRequest().body("Missing required field: user_id");
    }
    if alert_request.asset_id.is_empty() {
        return HttpResponse::BadRequest().body("Missing required field: asset_id");
    }
    if alert_request.price_alert.is_empty() {
        return HttpResponse::BadRequest().body("Missing required field: price_alert");
    }

    //check if price_alert is a valid number
    let price_alert = alert_request.price_alert.parse::<f64>();
    if price_alert.is_err() {
        return HttpResponse::BadRequest().body("price_alert must be a valid number");
    }

    alert_request.price_alert = price_alert.unwrap().to_string();

    let mut db_client = Client::connect(
        "host=localhost user=postgres  password=postgres dbname=postgres",
        NoTls,
    )
    .unwrap();

    let query = "SELECT id, notified  FROM alert_request WHERE user_id = $1 AND asset_id = $2 AND above = $3 AND price_alert = $4";

    //check if the query already existed
    let row = db_client
        .query(
            query,
            &[
                &alert_request.user_id,
                &alert_request.asset_id,
                &alert_request.above,
                &alert_request.price_alert,
            ],
        )
        .unwrap();

    if !row.is_empty() {
        let alert_request_id: i32 = row[0].get(0);
        let notified: bool = row[0].get(1);

        if notified {
            let query = "UPDATE alert_request SET notified = false WHERE id = $1";
            db_client
                .execute(query, &[&alert_request_id])
                .map_err(|e| {
                    eprintln!("Failed to execute query: {}", e);
                    HttpResponse::InternalServerError().finish()
                })
                .unwrap();

            return HttpResponse::Ok().json(alert_request_id);
        }

        return HttpResponse::BadRequest().body("Alert request already existed");
    }

    let query = "INSERT INTO alert_request (user_id, asset_id, above, price_alert) VALUES ($1, $2, $3, $4) RETURNING id";
    let row = db_client
        .query(
            query,
            &[
                &alert_request.user_id,
                &alert_request.asset_id,
                &alert_request.above,
                &alert_request.price_alert,
            ],
        )
        .unwrap();

    let alert_request_id: i32 = row[0].get(0);

    HttpResponse::Ok().json(alert_request_id)
}

#[post("/delete_alert_request")]
async fn delete_alert_request(alert_request: web::Json<DeleteAlertRequest>) -> impl Responder {
    let alert_request = alert_request.into_inner();

    // Validate the request body
    if alert_request.id == 0 {
        return HttpResponse::BadRequest().body("Missing required field: id");
    }

    let mut client = Client::connect(
        "host=localhost user=postgres  password=postgres dbname=postgres",
        NoTls,
    )
    .unwrap();

    let query = "DELETE FROM alert_request WHERE id = $1";

    let rows_affected = client
        .execute(query, &[&alert_request.id])
        .map_err(|e| {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    if rows_affected == 0 {
        HttpResponse::NotFound().finish()
    } else {
        HttpResponse::Ok().finish()
    }
}

#[post("/update_alert_request")]
async fn update_alert_request(alert_request: web::Json<UpdateAlertRequest>) -> impl Responder {
    let mut alert_request = alert_request.into_inner();

    // Validate the request body
    if alert_request.id == 0 {
        return HttpResponse::BadRequest().body("Missing required field: id");
    }
    if alert_request.user_id.is_empty() {
        return HttpResponse::BadRequest().body("Missing required field: user_id");
    }
    if alert_request.asset_id.is_empty() {
        return HttpResponse::BadRequest().body("Missing required field: asset_id");
    }
    if alert_request.price_alert.is_empty() {
        return HttpResponse::BadRequest().body("Missing required field: price_alert");
    }

    //check if price_alert is a valid number
    let price_alert = alert_request.price_alert.parse::<f64>();
    if price_alert.is_err() {
        return HttpResponse::BadRequest().body("price_alert must be a valid number");
    }

    alert_request.price_alert = price_alert.unwrap().to_string();

    let mut client = Client::connect(
        "host=localhost user=postgres password=postgres dbname=postgres",
        NoTls,
    )
    .unwrap();

    let query = "UPDATE alert_request SET user_id = $1, asset_id = $2, above = $3, price_alert = $4 WHERE id = $5";

    let rows_affected = client
        .execute(
            query,
            &[
                &alert_request.user_id,
                &alert_request.asset_id,
                &alert_request.above,
                &alert_request.price_alert,
                &alert_request.id,
            ],
        )
        .map_err(|e| {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    if rows_affected == 0 {
        HttpResponse::NotFound().finish()
    } else {
        HttpResponse::Ok().finish()
    }
}

#[get("/fetch_alert_requests")]
async fn fetch_alert_requests() -> impl Responder {
    let query = "SELECT * FROM alert_request";
    let mut db_client = Client::connect(
        "host=localhost user=postgres password=postgres dbname=postgres",
        NoTls,
    )
    .unwrap();
    let rows = db_client
        .query(query, &[])
        .map_err(|e| {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    let alert_requests = rows
        .iter()
        .map(|row| AlertRequest {
            id: row.get(0),
            user_id: row.get(1),
            asset_id: row.get(2),
            above: row.get(3),
            price_alert: row.get(4),
            notified: row.get(5),
        })
        .collect::<Vec<AlertRequest>>();

    HttpResponse::Ok().json(alert_requests)
}

#[post("/fetch_user_alert_requests")]
async fn fetch_user_alert_requests(
    alert_request: web::Json<FetchUserAlertRequest>,
) -> impl Responder {
    let query = "SELECT * FROM alert_request WHERE user_id = $1";
    let mut db_client = Client::connect(
        "host=localhost user=postgres password=postgres dbname=postgres",
        NoTls,
    )
    .unwrap();
    let rows = db_client
        .query(query, &[&alert_request.user_id])
        .map_err(|e| {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    let alert_requests = rows
        .iter()
        .map(|row| AlertRequest {
            id: row.get(0),
            user_id: row.get(1),
            asset_id: row.get(2),
            above: row.get(3),
            price_alert: row.get(4),
            notified: row.get(5),
        })
        .collect::<Vec<AlertRequest>>();

    //check if user has any alert requests
    if alert_requests.is_empty() {
        return HttpResponse::NotFound().finish();
    } else {
        return HttpResponse::Ok().json(alert_requests);
    }
}

#[get("/fetch_asset_prices")]
async fn fetch_asset_prices() -> impl Responder {
    let query = "SELECT * FROM asset_prices order by asset_id";
    let mut db_client = Client::connect(
        "host=localhost user=postgres  password=postgres dbname=postgres",
        NoTls,
    )
    .unwrap();
    let rows = db_client
        .query(query, &[])
        .map_err(|e| {
            eprintln!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    let asset_prices = rows
        .iter()
        .map(|row| AssetPrice {
            id: row.get(0),
            asset_id: row.get(1),
            price: row.get(2),
        })
        .collect::<Vec<AssetPrice>>();

    HttpResponse::Ok().json(asset_prices)
}

async fn init() -> Result<(), Box<dyn Error>> {
    let mut client = Client::connect(
        "host=localhost user=postgres  password=postgres dbname=postgres",
        NoTls,
    )?;
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS alert_request (
            id SERIAL PRIMARY KEY,
            user_id TEXT NOT NULL,
            asset_id TEXT NOT NULL,
            above BOOLEAN NOT NULL,
            price_alert TEXT NOT NULL,
            notified BOOLEAN NOT NULL DEFAULT false
          );
          
    ",
    )?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init().await.unwrap();
    HttpServer::new(|| {
        App::new()
            .service(add_alert_request)
            .service(delete_alert_request)
            .service(update_alert_request)
            .service(fetch_alert_requests)
            .service(fetch_user_alert_requests)
            .service(fetch_asset_prices)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
