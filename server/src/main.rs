#[tokio::main]
async fn main() {
    // let config_path = match env::args().nth(1) {
    //     Some(dir) => PathBuf::from(dir),
    //     None => return println!("conf error: \nargs filepath missing from args"),
    // };

    // let config = match config::Config::try_from(&config_path).await {
    //     Ok(conf) => conf,
    //     Err(e) => return println!("conf error:\n{}", e),
    // };

    // let available_encodings = AvailableEncodings::new(&config.content_encodings);

    // let listener = match TcpListener::bind(&config.host_and_port).await {
    //     Ok(lstnr) => lstnr,
    //     Err(e) => return println!("tcp listener error:\n{}", e),
    // };

    // loop {
    //     let (stream, _remote_address) = match listener.accept().await {
    //         Ok(strm) => strm,
    //         _ => {
    //             // log socket errors here
    //             continue;
    //         }
    //     };

    //     let io = TokioIo::new(stream);
    //     let service = service::Svc::new(&config, &available_encodings);

    //     tokio::task::spawn(async move {
    //         // log service errors here
    //         Builder::new(TokioExecutor::new())
    //             .serve_connection(io, service)
    //             .await
    //     });
    // }
}