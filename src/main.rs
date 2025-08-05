use axum::Router;
use dotenv::dotenv;
use ms_walrus::routes;
#[tokio::main]
async fn main() {
    dotenv().ok();
    let address = std::env::var("ADDRESS").unwrap_or("0.0.0.0".to_string());
    let port = std::env::var("PORT").unwrap_or("3000".to_string());
    let target_url = format!("{}:{}", address, port);

    println!("Running server at {}", target_url);
    start_server(target_url).await.unwrap();
}

async fn start_server(target_url: String) -> Result<(), std::io::Error> {
    let router = Router::new().merge(routes::router());
    let listener = tokio::net::TcpListener::bind(target_url).await?;
    axum::serve(listener, router).await?;
    Ok(())
}
