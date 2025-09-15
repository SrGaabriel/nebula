mod routing;

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use crate::app::NebulaApp;

pub async fn serve(app: NebulaApp) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "example_websockets=debug,tower_http=info,diesel=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

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