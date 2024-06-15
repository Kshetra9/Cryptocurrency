use actix_cors::Cors;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use bitcoincore_rpc::{Auth, Client, RpcApi};
use rusqlite::{params, Connection, Result};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use tokio::time::{interval, Duration};
use env_logger;

#[derive(Serialize, Deserialize)]
struct BlockchainData {
    block_height: i32,
    network_hash_rate: f64,
    difficulty: f64,
    mempool_size: i32,
}

struct AppState {
    db: Mutex<Connection>,
    bitcoin_rpc: Client,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();  // Initialize the logger

    // Initialize the database connection
    let conn = Connection::open("blockchain.db").expect("Failed to connect to database");

    // Create the table if it doesn't exist
    conn.execute(
        "CREATE TABLE IF NOT EXISTS blockchain (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            block_height INTEGER,
            network_hash_rate REAL,
            difficulty REAL,
            mempool_size INTEGER,
            timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).expect("Failed to create table");

    // Setup Bitcoin Core RPC client
    let rpc_url = "http://localhost:8332";
    let rpc_auth = Auth::UserPass("kshetra".to_string(), "kshetra".to_string());
    let bitcoin_rpc = Client::new(rpc_url, rpc_auth).expect("Failed to create RPC client");

    // Wrap the connection and RPC client in AppState
    let app_state = web::Data::new(AppState {
        db: Mutex::new(conn),
        bitcoin_rpc,
    });

    // Start the background task for automatic ingestion
    let app_state_clone = app_state.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(60));
        loop {
            interval.tick().await;
            if let Err(e) = fetch_and_ingest_background(&app_state_clone).await {
                eprintln!("Failed to fetch and ingest data: {:?}", e);
            }
        }
    });

    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
            )
            .route("/", web::get().to(index))
            .route("/ingest", web::post().to(ingest))
            .route("/fetch_and_ingest", web::post().to(fetch_and_ingest))
            .route("/fetch_metrics", web::get().to(fetch_metrics))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

async fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the Blockchain Server")
}

async fn ingest(data: web::Json<BlockchainData>, app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap();

    // Insert the data into the blockchain table
    let result = db.execute(
        "INSERT INTO blockchain (block_height, network_hash_rate, difficulty, mempool_size) VALUES (?1, ?2, ?3, ?4)",
        params![data.block_height, data.network_hash_rate, data.difficulty, data.mempool_size],
    );

    match result {
        Ok(_) => HttpResponse::Ok().json("Data ingested successfully"),
        Err(err) => {
            eprintln!("Failed to insert data: {:?}", err);
            HttpResponse::InternalServerError().json("Failed to ingest data")
        }
    }
}

async fn fetch_and_ingest(app_state: web::Data<AppState>) -> impl Responder {
    match fetch_blockchain_data(&app_state.bitcoin_rpc).await {
        Ok(data) => {
            let db = app_state.db.lock().unwrap();
            let result = db.execute(
                "INSERT INTO blockchain (block_height, network_hash_rate, difficulty, mempool_size) VALUES (?1, ?2, ?3, ?4)",
                params![data.block_height, data.network_hash_rate, data.difficulty, data.mempool_size],
            );

            match result {
                Ok(_) => HttpResponse::Ok().json("Data fetched and ingested successfully"),
                Err(err) => {
                    eprintln!("Failed to insert data: {:?}", err);
                    HttpResponse::InternalServerError().json("Failed to ingest data")
                }
            }
        },
        Err(err) => {
            eprintln!("Failed to fetch data from Bitcoin Core: {:?}", err);
            HttpResponse::InternalServerError().json("Failed to fetch data from Bitcoin Core")
        }
    }
}

async fn fetch_and_ingest_background(app_state: &web::Data<AppState>) -> Result<(), Box<dyn std::error::Error>> {
    match fetch_blockchain_data(&app_state.bitcoin_rpc).await {
        Ok(data) => {
            let db = app_state.db.lock().unwrap();
            db.execute(
                "INSERT INTO blockchain (block_height, network_hash_rate, difficulty, mempool_size) VALUES (?1, ?2, ?3, ?4)",
                params![data.block_height, data.network_hash_rate, data.difficulty, data.mempool_size],
            )?;
            Ok(())
        },
        Err(err) => {
            eprintln!("Failed to fetch data from Bitcoin Core: {:?}", err);
            Err(Box::new(err))
        }
    }
}

async fn fetch_blockchain_data(rpc: &Client) -> Result<BlockchainData, bitcoincore_rpc::Error> {
    let block_height = rpc.get_block_count()? as i32;
    let network_hash_rate = rpc.get_network_hash_ps(None, None)? as f64;
    let difficulty = rpc.get_difficulty()? as f64;
    let mempool = rpc.get_raw_mempool()?;
    let mempool_size = mempool.len() as i32;

    Ok(BlockchainData {
        block_height,
        network_hash_rate,
        difficulty,
        mempool_size,
    })
}

async fn fetch_metrics(app_state: web::Data<AppState>) -> impl Responder {
    let db = app_state.db.lock().unwrap();
    let mut stmt = db.prepare("SELECT block_height, network_hash_rate, difficulty, mempool_size FROM blockchain ORDER BY timestamp DESC LIMIT 1").unwrap();
    let mut rows = stmt.query([]).unwrap();

    if let Some(row) = rows.next().unwrap() {
        let data = BlockchainData {
            block_height: row.get(0).unwrap(),
            network_hash_rate: row.get(1).unwrap(),
            difficulty: row.get(2).unwrap(),
            mempool_size: row.get(3).unwrap(),
        };
        HttpResponse::Ok().json(data)
    } else {
        HttpResponse::NotFound().finish()
    }
}
