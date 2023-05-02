use tokio::io::{AsyncReadExt, AsyncWriteExt};

const BUFFER_SIZE: usize = 16 * 1024;
const DOCKER_HOST: &str = "127.0.0.1:2375";

async fn proxy<TR, TW>(mut reader: TR, mut writer: TW, label: &str) -> color_eyre::eyre::Result<()>
where
    TR: tokio::io::AsyncRead + Unpin,
    TW: tokio::io::AsyncWrite + Unpin,
{
    let mut buf = [0; BUFFER_SIZE];

    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 {
            break;
        }

        let d = &buf[0..n];

        log::trace!(
            "[{}] {:?}",
            label.to_uppercase(),
            String::from_utf8_lossy(d)
        );

        writer.write_all(d).await?;
    }

    log::info!("Connection closed.");

    writer.shutdown().await?;

    Ok(())
}

pub async fn handle_connection(
    conn: tokio::net::TcpStream,
    addr: &std::net::SocketAddr,
) -> color_eyre::eyre::Result<()> {
    let addr = addr.clone();

    let downstream = tokio::net::TcpStream::connect(DOCKER_HOST).await.unwrap();

    log::info!("[{addr}] Opened TCP socket to Docker daemon");

    let (ds_read, ds_write) = downstream.into_split();
    let (us_read, us_write) = conn.into_split();

    let us_proxy =
        tokio::spawn(async move { proxy(us_read, ds_write, &format!("{addr} upstream")).await });
    let ds_proxy =
        tokio::spawn(async move { proxy(ds_read, us_write, &format!("{addr} downstream")).await });

    ds_proxy.await??;
    us_proxy.await??;

    Ok(())
}
