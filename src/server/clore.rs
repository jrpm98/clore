use futures::executor::block_on;
use model::resent::ResentWeb;
#[allow(dead_code)]
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};
use serde_json::{Number, Value};
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    sync::Arc,
};
use tracing::{error, info};

use self::model::{resent::Resent, Card};
use crate::{
    config::{self, CONFIG},
    server::clore::model::{market::Marketplace, my_orders::MyOrders, wallet::Wallets},
};

pub mod model;
pub struct Clore {}

impl Default for Clore {
    fn default() -> Self {
        Self {}
    }
}

impl Clore {
    pub async fn marketplace(&self) -> Result<Vec<Card>, String> {
        let config::Clore { api_host, .. } = Clore::get_config().await;
        let url = format!("{}{}", api_host, "v1/marketplace");
        let text = Clore::get_client()
            .map_err(|e| e.to_string())?
            .get(url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;
        // info!("服务器响应:{:?}", &text);
        let blocked_server_ids = Clore::import_block_server_ids();
        let markets = serde_json::from_str::<Marketplace>(&text)
            .map_err(|e| e.to_string())?
            .filter()
            .iter()
            .filter(|card| !blocked_server_ids.contains(&card.server_id))
            .map(|card| card.clone())
            .collect::<Vec<_>>();
        // info!("可用卡:{:?}", &markets);
        Ok(markets)
    }

    pub async fn wallet(&self) -> Result<f64, String> {
        let config::Clore { api_host, .. } = Clore::get_config().await;
        let url = format!("{}{}", api_host, "v1/wallets");
        let text = Clore::get_client()
            .map_err(|e| e.to_string())?
            .get(url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;

        let wallets = text.parse::<Wallets>()?;
        let balance = wallets.filter();
        info!(text, balance);
        Ok(balance)
    }

    pub async fn create_order(&self, card: &Card, address: Vec<String>) -> Result<(), String> {
        let config::Clore {
            api_host,
            ssh_passwd,
            command,
            ..
        } = Clore::get_config().await;
        let url = format!("{}{}", api_host, "v1/create_order");
        let command = command
            .replace("{server_id}", card.server_id.to_string().as_str())
            .replace("{card_number}", card.card_number.to_string().as_str())
            .replace("{address}", address.join(",").as_str());
        let body = Resent::new(card.server_id, ssh_passwd, command);
        info!("body:{}", serde_json::to_string(&body).unwrap());
        let mut headers: HashMap<_, _> = HashMap::new();
        headers.insert("Content-type", HeaderValue::from_str("application/json"));
        let text = Clore::get_client()
            .map_err(|e| e.to_string())?
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;
        info!("{:?}", &text);
        let result = serde_json::from_str::<Value>(&text).map_err(|e| e.to_string())?;
        let code = result.get("code").map_or(-1i64, |val| {
            val.as_number()
                .unwrap_or(&Number::from(-1))
                .as_i64()
                .unwrap_or(-1)
        });

        if code == 0 {
            Ok(())
        } else {
            Err(format!("创建服务器失败，错误码:{:?}", code))
        }
    }

    pub async fn create_order_web_api(card: &Card, address: Vec<String>) -> Result<(), String> {
        let config::Clore {
            web_api_host,
            web_token,
            ssh_passwd,
            command,
            ..
        } = Clore::get_config().await;
        let url = format!("{}{}", web_api_host, "webapi/create_order");
        let command = command
            .replace("{server_id}", card.server_id.to_string().as_str())
            .replace("{card_number}", card.card_number.to_string().as_str())
            .replace("{address}", address.join(",").as_str());
        let resent = ResentWeb::new(card.server_id, ssh_passwd, web_token, command.clone());
        let client = Clore::get_client().map_err(|e| e.to_string())?;
        info!("command:{:?}", command.clone());
        let text = client
            .post(url)
            .json(&resent)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;
        info!("{}", text);
        if text.contains("completed") {
            info!("下单成功！");
            Ok(())
        } else {
            error!("下单失败:{:?}", text);
            Err(text)
        }
    }

    pub async fn my_orders(&self) -> Result<MyOrders, String> {
        let config::Clore { api_host, .. } = Clore::get_config().await;
        let url = format!("{}{}", api_host, "v1/my_orders");
        let text = Clore::get_client()
            .map_err(|e| e.to_string())?
            .get(url)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;
        info!("my_order_text:{}", text);
        let result: Result<MyOrders, String> =
            serde_json::from_str::<MyOrders>(&text).map_err(|e| e.to_string());
        info!("获取到订单号:{:?}", result);
        result
    }

    pub async fn cancel_order(&self, order_id: u32) -> Result<(), String> {
        let config::Clore { api_host, .. } = Clore::get_config().await;
        let url = format!("{}{}", api_host, "v1/cancel_order");
        let body = format!("\"{{\"id\":{}}}\"", order_id);
        let text = Clore::get_client()
            .map_err(|e| e.to_string())?
            .post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| e.to_string())?
            .text()
            .await
            .map_err(|e| e.to_string())?;
        let result = serde_json::from_str::<Value>(&text).map_err(|e| e.to_string())?;
        let code = result.get("code").map_or(-1i64, |val| {
            val.as_number()
                .unwrap_or(&Number::from(-1))
                .as_i64()
                .unwrap_or(-1)
        });

        if code == 0 {
            Ok(())
        } else {
            Err(format!("取消失败:{:?}", code))
        }
    }

    fn get_client() -> Result<Client, reqwest::Error> {
        let config::Clore { api_token, .. } = block_on(Clore::get_config());
        let token = api_token.clone();
        let mut headers = HeaderMap::new();
        headers.insert("auth", HeaderValue::from_str(&token).unwrap());
        ClientBuilder::new()
            .default_headers(headers)
            .timeout(std::time::Duration::from_secs(30))
            .build()
    }

    pub async fn get_config() -> config::Clore {
        let mutex_conf = Arc::clone(&CONFIG);
        let config = &mutex_conf.lock().await;
        (*config).clore.clone()
    }

    pub fn import_block_server_ids() -> Vec<u32> {
        let mut black_server_ids = Vec::<u32>::new();
        let openfile = Clore::open_block_file();
        if openfile.is_err() {
            error!("无法导入拉黑文件!");
            return black_server_ids;
        }
        let mut ids = String::from("");
        let mut reader = std::io::BufReader::new(openfile.unwrap());
        let _ = reader.read_to_string(&mut ids);
        black_server_ids = ids
            .split("\n")
            .into_iter()
            .map(|item| item.trim().parse::<u32>().unwrap_or_default())
            .collect::<Vec<u32>>();
        info!("黑名单:{:?}", black_server_ids);
        black_server_ids
    }

    pub fn append_block_server_id(server_id: u32) -> bool {
        let block_server_ids = Clore::import_block_server_ids();

        let openfile = Clore::open_block_file();
        if openfile.is_err() {
            error!("无法导出拉黑文件!");
            return false;
        }
        if !block_server_ids.contains(&server_id) {
            let mut writer = std::io::BufWriter::new(openfile.unwrap());
            let _ = writer.write_all(format!("\n{}", server_id).as_bytes());
        }
        true
    }

    fn open_block_file() -> Result<File, String> {
        let dir = std::env::current_dir().map_err(|e| e.to_string())?;
        let file = dir.join("block_server_ids.txt");
        let file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .read(true)
            .write(true)
            .open(file)
            .map_err(|e| e.to_string())?;
        Ok(file)
    }
}
