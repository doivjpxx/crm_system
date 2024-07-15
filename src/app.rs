use crate::router;

pub async fn run_app() {
    let listener = tokio::net::TcpListener::bind(&"0.0.0.0:3000")
        .await
        .unwrap();

    let router = router::create_router();

    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();
}
