use std::str::FromStr;

use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};

#[repr(u8)]
#[derive(
    Debug, PartialEq, EnumString, Display, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize,
)]
pub enum CardType {
    NVIDIA4090 = 1,
    NVIDIA4080S = 2,
    NVIDIA4080 = 3,
    NVIDIA4070S = 4,
    NVIDIA4070 = 5,
    NVIDIA4070TI = 6,
    NVIDIA3090 = 7,
    NVIDIA3090TI = 8,
    NVIDIA3080TI = 9,
    NVIDIA3080 = 10,
    NVIDIA1080TI = 11,
    NVIDIA1080 = 12,
    UNKNOWN(String),
}

impl CardType {
    pub fn get_max_price(&self, card_number: f64) -> f64 {
        let price = match self {
            CardType::NVIDIA4090 => 32f64,
            CardType::NVIDIA4080S => 24f64,
            CardType::NVIDIA4080 => 20f64,
            CardType::NVIDIA4070S => 32f64,
            CardType::NVIDIA4070 => 17f64,
            CardType::NVIDIA4070TI => 17f64,
            CardType::NVIDIA3090 => 19f64,
            CardType::NVIDIA3090TI => 19f64,
            CardType::NVIDIA3080TI => 15f64,
            CardType::NVIDIA3080 => 15f64,
            CardType::NVIDIA1080TI => 10f64,
            CardType::NVIDIA1080 => 10f64,
            CardType::UNKNOWN(_) => 0f64,
        };

        price * card_number
    }
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Currency {
    #[serde(rename(serialize = "bitcoin"))]
    BITCOIN,
    #[serde(rename(serialize = "CLORE-Blockchain"))]
    CLORE,
}

impl FromStr for Currency {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "bitcoin" => Ok(Currency::BITCOIN),
            "CLORE-Blockchain" => Ok(Currency::CLORE),
            _ => Err("没有此货币".to_string()),
        }
    }
}

impl ToString for Currency {
    fn to_string(&self) -> String {
        match &self {
            Currency::BITCOIN => "bitcoin".to_string(),
            Currency::CLORE => "CLORE-Blockchain".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Card {
    pub server_id: u32,
    pub avg_score: f64,
    pub price_demand: f64,
    pub avg_price_demand: f64,
    pub price_spot: f64,
    pub avg_price_spot: f64,
    pub mrl: u32,
    pub card_number: i32,
    pub rented: bool,
    pub card_type: CardType,
}

pub mod market {
    use std::{
        collections::HashMap,
        ops::{Deref, DerefMut},
        str::FromStr,
    };

    use regex::Regex;
    use serde::{Deserialize, Serialize};
    use tracing::warn;

    use super::{Card, CardType};

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Net {
        up: f64,
        down: f64,
        cc: String,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Specs {
        pub mb: String,
        pub cpu: String,
        pub cpus: String,
        pub ram: f64,
        pub disk: String,
        pub disk_speed: f32,
        pub gpu: String,
        pub gpuram: f32,
        pub net: Net,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Price {
        pub on_demand: HashMap<String, f64>,
        pub spot: HashMap<String, f64>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Server {
        pub allowed_coins: Vec<String>,
        pub id: u32,
        pub owner: u32,
        pub mrl: u32,
        pub price: Price,
        pub rented: bool,
        pub specs: Specs,
        pub rating: HashMap<String, f32>,
    }

    #[derive(Serialize, Deserialize, Debug, Clone)]
    pub struct Marketplace {
        servers: Vec<Server>,
        my_servers: Vec<u32>,
        code: u32,
    }

    impl Marketplace {
        pub fn filter(&self) -> Vec<Card> {
            let regex = Regex::new(r"(3080|3090|4070|4080|4080|4090)").unwrap();
            let mut cards: Vec<Card> = (*self)
                .iter()
                .filter(|item| {
                    let machine_properties = &item.specs;
                    let gpu = &machine_properties.gpu;
                    regex.is_match(&gpu)
                        && item.rating.get("avg").unwrap_or(&0f32) > &3.5f32
                        && item.allowed_coins.contains(&"CLORE-Blockchain".to_string())
                        && !item.rented
                        && item.mrl > 72
                        && item.specs.net.down > 20f64
                })
                .map(|item| {
                    let card_info = item
                        .specs
                        .gpu
                        .split(' ')
                        .map(|item| item.to_string())
                        .collect::<Vec<String>>();
                    let number = card_info.get(0).map_or(0, |s| {
                        let s = s.replace("x", "");
                        s.parse::<i32>().map_or(1, |n| n)
                    });
                    let factory = card_info
                        .get(1)
                        .map(|item| item.to_owned())
                        .unwrap_or_default();
                    let card_type = card_info
                        .get(4)
                        .map(|item| item.to_owned())
                        .unwrap_or_default();
                    let mut flag = card_info
                        .get(5)
                        .map(|itme| itme.to_uppercase().to_owned())
                        .unwrap_or_default();
                    flag = match flag.as_str() {
                        "TI" => "TI".to_string(),
                        "SUPER" => "S".to_string(),
                        _ => "".to_string(),
                    };
                    let card_type =
                        CardType::from_str(&format!("{}{}{}", factory, card_type, flag))
                            .unwrap_or_else(|_| CardType::UNKNOWN(card_info.join(" ")));
                    let price_demand = item
                        .price
                        .on_demand
                        .get("CLORE-Blockchain")
                        .and_then(|price| Some(price.clone()))
                        .unwrap_or_default();
                    let avg_price_demand = price_demand / (number as f64);

                    let price_spot = item
                        .price
                        .spot
                        .get("CLORE-Blockchain")
                        .and_then(|price| Some(price.clone()))
                        .unwrap_or_default();
                    let avg_price_spot = price_spot / (number as f64);
                    let avg_score = item.rating.get("avg").unwrap_or(&0f32);
                    let avg_score = *avg_score as f64;
                    let card = Card {
                        server_id: item.id,
                        avg_score: avg_score,
                        price_demand: price_demand,
                        avg_price_demand: avg_price_demand,
                        price_spot: price_spot,
                        avg_price_spot: avg_price_spot,
                        mrl: item.mrl,
                        card_number: number,
                        rented: item.rented,
                        card_type: card_type,
                    };
                    card
                })
                .filter(|item| {
                    let total_max_price = item.card_type.get_max_price(item.card_number as f64);
                    match item.card_type {
                        CardType::UNKNOWN(_) => {
                            warn!("未知显卡:{:?}", item.card_type);
                            false
                        }
                        _ if total_max_price > item.avg_price_demand => true,
                        _ => false,
                    }
                })
                .collect();
            cards.sort_by(|a, b| b.card_type.cmp(&a.card_type));
            cards.reverse();
            for item in cards.iter() {
                let log = format!(
                    "服务器id:{:>5},显卡型号:{:>12},用户评分：{:.1},显卡数量:{:2},卖家价格:{:>3.3},卖家均价:{:>3.3},买家出价:{:>3.3},买家均价:{:>3.3}",
                    item.server_id,
                    item.card_type,
                    item.avg_score,
                    item.card_number,
                    item.price_demand,
                    item.avg_price_demand,
                    item.card_type.get_max_price(item.card_number.clone() as f64),
                    item.card_type.get_max_price(1f64)
                );
                println!("{:?}", log);
            }
            cards
        }
    }

    impl Deref for Marketplace {
        type Target = Vec<Server>;

        fn deref(&self) -> &Self::Target {
            &self.servers
        }
    }

    impl DerefMut for Marketplace {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.servers
        }
    }
}

pub mod wallet {
    use std::str::FromStr;

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Wallet {
        pub name: String,
        pub deposit: String,
        pub balance: f64,
        pub withdrawal_fee: f64,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Wallets {
        pub wallets: Vec<Wallet>,
        pub code: i32,
    }

    impl FromStr for Wallets {
        type Err = String;

        fn from_str(s: &str) -> Result<Wallets, Self::Err> {
            let wallets = serde_json::from_str::<Wallets>(s).map_err(|e| e.to_string())?;
            Ok(wallets)
        }
    }

    impl Wallets {
        pub fn filter(&self) -> f64 {
            let mut balance = 0f64;
            for wallet in self.wallets.iter() {
                if wallet.name == "CLORE-Blockchain" {
                    balance = wallet.balance;
                    break;
                }
            }
            balance
        }
    }
}

pub mod resent {
    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    use super::Currency;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Resent {
        currency: Currency,
        image: String,
        renting_server: u32,
        #[serde(rename(serialize = "type"))]
        demand: String,
        ports: HashMap<String, String>,
        env: HashMap<String, String>,
        ssh_password: String,
        command: String,
    }

    impl Resent {
        pub fn new(server_id: u32, ssh_passwd: String, command: String) -> Resent {
            let mut ports = HashMap::<String, String>::new();

            ports.insert("22".to_string(), "tcp".to_string());
            ports.insert("8888".to_string(), "http".to_string());
            Self {
                currency: Currency::CLORE,
                image: "cloreai/torch:2.0.1".to_string(),
                renting_server: server_id,
                demand: "on-demand".to_string(),
                ports: ports,
                env: Default::default(),
                ssh_password: ssh_passwd,
                command: command,
            }
        }
    }

    impl ToString for Resent {
        fn to_string(&self) -> String {
            serde_json::to_string(&self).unwrap_or_default()
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ResentWeb {
        currency: Currency,
        image: String,
        dockerhub_auth: String,
        ports: HashMap<String, String>,
        env: HashMap<String, String>,
        #[serde(rename(serialize = "type"))]
        demand: String,
        renting_server: u32,
        remember_password: bool,
        token: String,
        command: String,
    }

    impl ResentWeb {
        pub fn new(
            server_id: u32,
            ssh_passwd: String,
            web_token: String,
            command: String,
        ) -> ResentWeb {
            let mut envs = HashMap::<String, String>::new();
            // {"WEBUI_PASSWORD":"MTcxNjMwNDc2N19ZempBSW","SSH_PASSWORD":"MTcxNjMwNDc2N19ZempBSW"}
            envs.insert("WEBUI_PASSWORD".to_string(), ssh_passwd.clone());
            envs.insert("SSH_PASSWORD".to_string(), ssh_passwd.clone());
            let mut ports = HashMap::<String, String>::new();

            ports.insert("22".to_string(), "tcp".to_string());
            ports.insert("8888".to_string(), "http".to_string());
            ResentWeb {
                currency: Currency::CLORE,
                image: "cloreai/torch:2.0.1".to_string(),
                dockerhub_auth: "".to_string(),
                ports,
                env: envs,
                demand: "on-demand".to_string(),
                renting_server: server_id,
                remember_password: true,
                token: web_token.to_string(),
                command,
            }
        }
    }
}

pub mod my_orders {
    use std::ops::{Deref, DerefMut};

    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Order {
        #[serde(alias = "id")]
        pub order_id: u32,
        #[serde(alias = "si")]
        pub server_id: u32,
        pub pub_cluster: Vec<String>,
        pub tcp_ports: Vec<String>,
        pub http_port: String,
    }

    impl Order {
        pub fn get_map_ssh_port(&self) -> Option<u16> {
            let mut ssh_map_port = None;
            for map in self.tcp_ports.iter() {
                let port_maps = map
                    .split(":")
                    .map(|e| e.parse::<u16>().unwrap_or_default())
                    .collect::<Vec<u16>>();
                match port_maps[..] {
                    [origin, map] if origin == 22 => {
                        ssh_map_port = Some(map);
                    }
                    _ => {
                        ssh_map_port = None;
                    }
                };
                if ssh_map_port.is_some() {
                    break;
                }
            }

            ssh_map_port
        }

        pub fn get_ssh_host(&self) -> Option<String> {
            let index = self.pub_cluster.len();
            if index > 0 {
                self.pub_cluster
                    .get(index - 1)
                    .and_then::<String, _>(|host| Some(host.clone()))
            } else {
                None
            }
        }
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct MyOrders {
        code: i32,
        orders: Vec<Order>,
    }

    impl Deref for MyOrders {
        type Target = Vec<Order>;

        fn deref(&self) -> &Self::Target {
            &self.orders
        }
    }

    impl DerefMut for MyOrders {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.orders
        }
    }

    impl MyOrders {
        pub fn filter(&self) {}
    }
}
