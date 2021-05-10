use hmac_sha256::Hash;
use serde_json::{Value, json};

use hyper_tls::HttpsConnector;
use hyper::{Client, Request, Method, Body};
use hyper::client::connect::HttpConnector;

use std::time::{SystemTime, UNIX_EPOCH};
use std::str;

use super::jwt;

extern crate hex;
extern crate urlencoding;

type UpbitApiResult = Result<Value, Box<dyn std::error::Error + Send + Sync>>;

pub struct Upbit {
    access: String,
    secret: String,
    client: Client<HttpsConnector<HttpConnector>, Body>,
    base: String,
}

impl Upbit {

    pub fn new(access: &String, secret: &String) -> Upbit {
        Upbit {
            access: access.to_string(),
            secret: secret.to_string(),
            client: Client::builder().build::<_, hyper::Body>(HttpsConnector::new()),
            base: "https://api.upbit.com".to_string()
        }
    }

}

impl Upbit {

    fn now(&self) -> u64 {
        let since_the_epoch = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards");
        return since_the_epoch.as_secs();
    }

    fn hash(&self, value: Value) -> String {
        let mut hash_list = Vec::new();
        for (key, val) in value.as_object().unwrap() {
            if val.is_array() {
                for v in val.as_array().unwrap() {
                    hash_list.push(format!("{}[]={}", key, urlencoding::encode(v.as_str().unwrap())));
                }
            } else {
                hash_list.push(format!("{}={}", key, urlencoding::encode(val.as_str().unwrap())));
            }
        }
        return hash_list.join("&");
    }

    fn token(&self, value: Option<Value>) -> String {
        let mut payload = json!({
            "access_key": self.access,
            "nonce": "deviceUUID",
            "iat": self.now()
        });

        if value != None {
            let mut hash = self.hash(value.unwrap());
            hash = hex::encode(Hash::hash(hash.as_bytes()));
            payload["query_hash"] = Value::String(hash);
            payload["query_hash_alg"] = Value::String("SHA256".to_string());
        }

        let jwt = jwt::signin(jwt::header::default(),
            payload, self.secret.to_string());
        return format!("Bearer {}", jwt);
    }

    async fn request(&self, url: &str, method: Method, value: Option<Value>) -> UpbitApiResult {
        let mut body = Body::from(r#""#);
        let mut u = format!("{}{}", self.base, url);
        let v = value.clone();

        if value != None {
            if method == Method::GET {
                u = format!("{}?{}", u, self.hash(v.unwrap()));
            } else {
                body = Body::from(v.unwrap().to_string());
            }
        }

        let req = Request::builder()
                .method(method)
                .uri(u)
                .header("Content-Type", "application/json")
                .header("Authorization", self.token(value))
                .body(body)?;

        let res = self.client.request(req).await?;
        let bytes = hyper::body::to_bytes(res.into_body()).await;
        let c: &[u8] = &bytes.unwrap().to_vec();
        let s = str::from_utf8(c).unwrap();

        Ok(serde_json::from_str(s)?)
    }
}

impl Upbit {

    pub async fn accounts(&self) -> UpbitApiResult {

        let _v = self.request("/v1/accounts", Method::GET, None).await?;
        Ok(_v)

    }

    pub async fn orders_chance(&self, market: String) -> UpbitApiResult {

        let _d = json!({
            "market": market,
        });
        let _v = self.request("/v1/orders/chance", Method::GET, Some(_d)).await?;
        Ok(_v)

    }

    pub async fn order(&self, uuid: String) -> UpbitApiResult {

        let _d = json!({
            "uuid": uuid,
        });
        let _v = self.request("/v1/order", Method::GET, Some(_d)).await?;
        Ok(_v)

    }

    pub async fn orders(&self, uuids: Vec<String>) -> UpbitApiResult {

        let _d = json!({
            "uuids": uuids,
        });
        let _v = self.request("/v1/orders", Method::GET, Some(_d)).await?;
        Ok(_v)

    }

    pub async fn cancel_order(&self, uuid: String) -> UpbitApiResult {

        let _d = json!({
            "uuid": uuid,
        });
        let _v = self.request("/v1/order", Method::DELETE, Some(_d)).await?;
        Ok(_v)

    }

    pub async fn request_order(&self, market: String, side: String, volume: u64, price: u64, ord_type: String) -> UpbitApiResult {

        let _d = json!({
            "market": market,
            "side": side,
            "volume": volume.to_string(),
            "price": price.to_string(),
            "ord_type": ord_type,
        });
        let _v = self.request("/v1/orders", Method::POST, Some(_d)).await?;
        Ok(_v)

    }

    pub async fn market_list(&self, is_details: bool) -> UpbitApiResult {

        let _d = json!({
            "isDetails": is_details.to_string(),
        });
        let _v = self.request("/v1/market/all", Method::GET, Some(_d)).await?;
        Ok(_v)

    }

    pub async fn candles_minutes(&self, unit: u8, market: String, count: u32) -> UpbitApiResult {

        let _d = json!({
            "market": market,
            "count": count.to_string(),
        });
        let _u = format!("/v1/candles/minutes/{}", unit);
        let _v = self.request(&_u.to_owned(), Method::GET, Some(_d)).await?;
        Ok(_v)

    }

    pub async fn candles_days(&self, market: String, count: u32) -> UpbitApiResult {

        let _d = json!({
            "market": market,
            "count": count.to_string(),
        });
        let _v = self.request("/v1/candles/days", Method::GET, Some(_d)).await?;
        Ok(_v)

    }

    pub async fn candles_weeks(&self, market: String, count: u32) -> UpbitApiResult {

        let _d = json!({
            "market": market,
            "count": count.to_string(),
        });
        let _v = self.request("/v1/candles/weeks", Method::GET, Some(_d)).await?;
        Ok(_v)

    }

    pub async fn candles_months(&self, market: String, count: u32) -> UpbitApiResult {

        let _d = json!({
            "market": market,
            "count": count.to_string(),
        });
        let _v = self.request("/v1/candles/months", Method::GET, Some(_d)).await?;
        Ok(_v)

    }

    pub async fn trades(&self, market: String, count: u32) -> UpbitApiResult {

        let _d = json!({
            "market": market,
            "count": count.to_string(),
        });
        let _v = self.request("/v1/trades/ticks", Method::GET, Some(_d)).await?;
        Ok(_v)

    }

    pub async fn ticker(&self, markets: Vec<String>) -> UpbitApiResult {

        let _d = json!({
            "markets": markets.join(","),
        });
        let _v = self.request("/v1/ticker", Method::GET, Some(_d)).await?;
        Ok(_v)

    }

    pub async fn orderbook(&self, markets: Vec<String>) -> UpbitApiResult {

        let _d = json!({
            "markets": markets.join(","),
        });
        let _v = self.request("/v1/orderbook", Method::GET, Some(_d)).await?;
        Ok(_v)

    }

}
