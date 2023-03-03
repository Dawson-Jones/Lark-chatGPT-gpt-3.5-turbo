use std::net::SocketAddr;

use feishu_GPTbot::feishu;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let app = feishu::route_init();
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    feishu::run(app, addr).await;
}

// TODO: 定时清理 map
// fn
