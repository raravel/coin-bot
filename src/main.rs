use std::env;
use tokio::runtime::Runtime;

pub mod upbit;
pub mod jwt;

use upbit::Upbit;

async fn dummy() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("a few arguments");
        return;
    }

    let access_key = &args[1];
    let secret_key = &args[2];

    let up = Upbit::new(access_key, secret_key);
    //let val = up.accounts().await;
    //let val = up.orders_chance("KRW-BTC".to_string()).await;
    //let val = up.ticker(vec!["KRW-BTC".to_string()]).await;
    //let val = up.candles_minutes(1, "KRW-BTC".to_string(), 5).await;
    let val = up.market_list(true).await;

    println!("{}", val.unwrap().to_string());
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(dummy());
}
