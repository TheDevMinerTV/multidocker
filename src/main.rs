mod docker;
mod proxy;

#[tokio::main]
async fn main() -> color_eyre::eyre::Result<()> {
    pretty_env_logger::init();
    color_eyre::install()?;

    let daemon1 = docker::client::Client::init("http://127.0.0.1:2375".to_string())
        .await
        .unwrap();

    let proxy_tcp_server = tokio::net::TcpListener::bind(("0.0.0.0", 2377)).await?;

    tokio::spawn(async move {
        log::info!("Docker proxy listening on 0.0.0.0:2377");

        loop {
            match proxy_tcp_server.accept().await {
                Ok((conn, addr)) => {
                    tokio::spawn(async move {
                        log::debug!("[{addr}] Incoming connection");

                        match proxy::handle_connection(conn, &addr).await {
                            Ok(_) => log::info!("[{addr}] Connection closed."),
                            Err(e) => log::error!("[{addr}] Error: {}", e),
                        }
                    });
                }
                Err(e) => {
                    log::error!("Error accepting connection: {}", e);
                    break;
                }
            }
        }
    });

    let http_tcp_server = tokio::net::TcpListener::bind(("0.0.0.0", 2378)).await?;
    let app = axum::Router::new()
        .route("/_ping", axum::routing::any(docker::handler::ping))
        .route("/v1.42/info", axum::routing::get(docker::handler::info))
        .with_state(docker::handler::State {
            clients: vec![daemon1],
        });

    tokio::spawn(async move {
        log::info!("HTTP proxy listening on 0.0.0.0:2378");
        axum::serve(http_tcp_server, app).await.unwrap();
    });

    tokio::signal::ctrl_c().await?;

    Ok(())
}
