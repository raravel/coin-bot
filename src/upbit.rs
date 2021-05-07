use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};
use serde_json::{Value};

use hyper_tls::HttpsConnector;
use hyper::{Client, Request, Method, Body};
use hyper::client::connect::HttpConnector;

use std::str;

use super::jwt;

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

    fn token(&self) -> String {
        let jwt = jwt::signin(jwt::header::default(),
            json!({
                "access_key": self.access,
                "nonce": "deviceUUID",
                "iat": self.now()
            }), self.secret.to_string());
        return format!("Bearer {}", jwt);
    }

    fn eb(&self) -> Body {
        return Body::from(r#""#);
    }

    async fn request(&self, url: &str, method: Method, body: Body) -> UpbitApiResult {
        let u = format!("{}{}", self.base, url);
        let req = Request::builder()
                .method(method)
                .uri(u)
                .header("Content-Type", "application/json")
                .header("Authorization", self.token())
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

        let _v = self.request("/v1/accounts", Method::GET, self.eb()).await?;
        Ok(_v)

    }

}
