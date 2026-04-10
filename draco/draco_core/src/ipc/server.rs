use axum::{routing::post, Router};
use tokio::sync::mpsc::Sender;
use crate::core::types::{Job, Request};

pub async fn start_server(tx: Sender<Job>) {
    let app = Router::new().route(
        "/input",
        post(move |payload| handler(payload, tx.clone())),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    axum::serve(listener, app)
        .await
        .unwrap();
}

async fn handler(
    axum::Json(req): axum::Json<Request>,
    tx: Sender<Job>,
) -> String {
    let _ = tx.send(Job::UserRequest(req)).await;
    "ok".into()
}
