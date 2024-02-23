use axum::{response::Html, routing::get, Router};
use serde::Deserialize;
use symbol::HistoricalData;

pub mod symbol;

#[tokio::main]
async fn main() {

    let data = historical_data("CSV").await.unwrap();
    println!("{data:?}");

    let app = Router::new()
        .route("/", get(index))
        .route("/bootstrap.css", get(bootstrap_css))
        .route("/bootstrap.js", get(bootstrap_js));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn new_function_test1() {
    println!("this is a cool program yay!");
}

async fn index() -> Html<String> {
    Html(
        std::fs::read_to_string("frontend/index.html")
        .expect("no index.html exists")
    )
}


async fn bootstrap_css() -> String {
    std::fs::read_to_string("frontend/bootstrap/css/bootstrap.css")
    .expect("no index.html exists")
}

async fn bootstrap_js() -> String {
    std::fs::read_to_string("frontend/bootstrap/js/bootstrap.js")
    .expect("no index.html exists")
}

async fn historical_data(symbol: &str) -> reqwest::Result<Vec<HistoricalData>> {
    // https://query1.finance.yahoo.com/v7/finance/download/MATW?period1=0&period2=17085255820000&interval=1d&events=history&includeAdjustedClose=true

    let url = format!("https://query1.finance.yahoo.com/v7/finance/download/{symbol}?period1=0&period2=10000000000000000000&interval=1d&events=history&includeAdjustedClose=true");
    let csv_data = reqwest::get(url).await?.text().await?;
    let mut data = csv::Reader::from_reader(csv_data.as_bytes());
    let mut entries = Vec::new();
    for entry in data.deserialize() {
        let entry: HistoricalData = entry.expect("invalid csv data");
        entries.push(entry);
    }

    Ok(entries)
}