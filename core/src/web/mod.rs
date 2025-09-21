pub mod routing;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use crate::app::NebulaApp;

pub async fn serve(app: NebulaApp) {
    let listener = TcpListener::bind(app.config.rest_addr)
        .await
        .expect("Failed to bind rest API address");
    let router = routing::router(app);
    tracing::info!("Serving on {}", listener.local_addr().unwrap());
    axum::serve(
        listener,
        router.into_make_service_with_connect_info::<SocketAddr>()
    )
        .await
        .expect("Failed to run server");
}