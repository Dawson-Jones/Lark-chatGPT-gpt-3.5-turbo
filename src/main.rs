use feishu_gptbot::{feishu, CONFIG};
use std::net::{IpAddr, SocketAddr};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let ip: IpAddr = CONFIG.host.ip.parse().unwrap();
    let app = feishu::route_init();
    let addr = SocketAddr::from((ip, CONFIG.host.port));

    feishu::run(app, addr).await;
}

// TODO: 定时清理 map
// fn
