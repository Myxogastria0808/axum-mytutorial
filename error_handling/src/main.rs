use anyhow::bail;
use axum::{routing::get, Router};

mod my_error;
use my_error::app_error::AppError;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    //Router
    let app = Router::new().route("/app", get(app_handler));

    //Server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

//Handler
async fn app_handler() -> Result<String, AppError> {
    app_error_sample().await?;
    Ok("AppError Sample".to_string())
}

//app_handler Error
async fn app_error_sample() -> Result<(), anyhow::Error> {
    bail!("Error Sample");
}
