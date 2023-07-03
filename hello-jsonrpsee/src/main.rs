#[derive(Clone)]
struct SimpleLogger;

impl jsonrpsee::server::logger::Logger for SimpleLogger {
    type Instant = std::time::Instant;

    fn on_connect(
        &self,
        remote_addr: std::net::SocketAddr,
        request: &jsonrpsee::server::logger::HttpRequest,
        _t: jsonrpsee::server::logger::TransportProtocol,
    ) {
        println!(
            "[Logger::on_connect] remote_addr {:?}, request: {:?}",
            remote_addr, request
        );
    }

    fn on_request(&self, _t: jsonrpsee::server::logger::TransportProtocol) -> Self::Instant {
        Self::Instant::now()
    }

    fn on_call(
        &self,
        name: &str,
        params: jsonrpsee::server::logger::Params,
        kind: jsonrpsee::server::logger::MethodKind,
        _t: jsonrpsee::server::logger::TransportProtocol,
    ) {
        println!(
            "[Logger::on_call] method: '{}', params: {:?}, kind: {}",
            name, params, kind
        );
    }

    fn on_result(
        &self,
        name: &str,
        success: bool,
        _: Self::Instant,
        _t: jsonrpsee::server::logger::TransportProtocol,
    ) {
        println!("[Logger::on_result] method: '{}', success? {}", name, success);
    }

    fn on_response(
        &self,
        result: &str,
        started_at: Self::Instant,
        _t: jsonrpsee::server::logger::TransportProtocol,
    ) {
        println!(
            "[Logger::on_response] result: {}, time elapsed {:?}",
            result,
            started_at.elapsed()
        );
    }

    fn on_disconnect(
        &self,
        remote_addr: std::net::SocketAddr,
        _t: jsonrpsee::server::logger::TransportProtocol,
    ) {
        println!("[Logger::on_disconnect] remote_addr: {:?}", remote_addr);
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::FmtSubscriber::builder()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .try_init()
        .expect("setting default subscriber failed");
    let server_addr = run_server().await?;
    println!("Run the following snippet in the developer console in any website.");
    println!(
        r#"
        curl http://{} \ 
        -X POST \
        -H "Content-Type: application/json" \
        -d '{{"jsonrpc":"2.0","id":1,"method":"say_hello","params":["world"]}}'
        "#,
        server_addr
    );

    futures::future::pending().await
}

async fn run_server() -> anyhow::Result<std::net::SocketAddr> {
    let cors = tower_http::cors::CorsLayer::new()
        .allow_methods([hyper::Method::POST])
        .allow_origin(tower_http::cors::Any)
        .allow_headers([hyper::header::CONTENT_TYPE]);
    let middleware = tower::ServiceBuilder::new().layer(cors);
    let server = jsonrpsee::server::ServerBuilder::default()
        .set_host_filtering(jsonrpsee::server::AllowHosts::Any)
        .set_middleware(middleware)
        .set_logger(SimpleLogger)
        .build("127.0.0.1:0".parse::<std::net::SocketAddr>()?)
        .await?;
    let mut module = jsonrpsee::server::RpcModule::new(());
    module.register_method("say_hello", |params, _| {
        let name = params.one::<&str>().unwrap();
        format!("Hello there, {}!", name)
    })?;
    let addr = server.local_addr()?;
    let handle = server.start(module)?;
    tokio::spawn(handle.stopped());
    Ok(addr)
}
