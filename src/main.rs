use std::env;
use tokio::runtime::Runtime;
use std::{thread, time};
use colored::*;

pub mod upbit;
pub mod jwt;

use upbit::Upbit;

fn sleep(msec: u64) {
    let m = time::Duration::from_millis(msec);
    thread::sleep(m);
}

async fn dummy() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("a few arguments");
        return;
    }

    let access_key = &args[1];
    let secret_key = &args[2];

    let up = Upbit::new(access_key, secret_key);
    let mut val = up.accounts().await;
    let mut coin_vec = Vec::new();

    let mut my_krw: f64 = 0.0_f64;

    for v in val.unwrap().as_array().unwrap() {
        if v["currency"] == "KRW" {
            let balance = v["balance"].as_str().unwrap();
            my_krw = balance.parse::<f64>().unwrap();
            break;
        }
    }

    println!("현재 지갑 잔액은 {}원 입니다.", my_krw.to_string().yellow().bold());

    val = up.market_list(true).await;

    for v in val.unwrap().as_array().unwrap() {
        let market = v["market"].as_str().unwrap();
        if market.starts_with("KRW-") {
            if v["market_warning"] == "NONE" {
                let candles = up.candles_days(market, 30).await;
                let mut sum: f64 = 0.0_f64;

                let can = candles.unwrap();

                let ca = can.as_array().unwrap();
                if ca[0]["trade_price"].as_f64().unwrap() > my_krw / 2.0 {
                    sleep(100);
                    continue;
                }

                for c in can.as_array().unwrap() {
                    sum += c["change_rate"].as_f64().unwrap();
                }


                if sum >= -0.5_f64 && sum <= 1.0_f64 {
                    let mut ss = sum.to_string().red().bold();
                    if sum >= 0.0_f64 {
                        ss = sum.to_string().green().bold();
                    }
                    //println!("{}의 30일 가격변동 점수는 {}입니다.", v["korean_name"].as_str().unwrap().blue().bold(), ss);


                    let min_candles = up.candles_minutes(30, market, 200).await;
                    let min = min_candles.unwrap();
                    let min_arr = min.as_array().unwrap();
                    let len = min_arr.len() - 2;

                    sum = 0.0_f64;

                    for idx in 1..=len {
                        let now = min_arr[idx]["trade_price"].as_f64().unwrap();
                        let before = min_arr[idx+1]["trade_price"].as_f64().unwrap();
                        let devi = (now - before) / before * 100.0;
                        sum += devi;
                    }

                    ss = sum.to_string().red().bold();
                    if sum >= 0.0_f64 {
                        ss = sum.to_string().green().bold();
                        if sum >= 5.0_f64 && sum < 15.0_f64 {
                            coin_vec.push(v.clone());
                        }
                    }

                    //println!("{}의 10시간 가격변동 점수는 {}입니다.", v["korean_name"].as_str().unwrap().blue().bold(), ss);
                }
            }
        }
        sleep(200);
    }

    let buy_krw = my_krw / coin_vec.len();

    for coin in coin_vec {
        
        println!("{}", coin["korean_name"].as_str().unwrap());
    }

    //let val = up.orders_chance("KRW-BTC".to_string()).await;
    //let val = up.ticker(vec!["KRW-BTC".to_string()]).await;
    //let val = up.candles_minutes(1, "KRW-BTC".to_string(), 5).await;
    //let val = up.market_list(true).await;

    //println!("{}", val.unwrap().to_string());
}

fn main() {
    let rt = Runtime::new().unwrap();
    rt.block_on(dummy());
}
