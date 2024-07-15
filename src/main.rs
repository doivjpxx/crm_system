mod app;
mod db;
mod handlers;
mod router;

#[tokio::main]
async fn main() {
    app::run_app().await;
}
