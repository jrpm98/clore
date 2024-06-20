pub mod common;

#[cfg(test)]
mod test {

    use std::{
        any::{self, Any},
        io::Read,
    };

    use monitor::server::{
        clore::{
            model::{market::Marketplace, Card},
            Clore,
        },
        ssh::Ssh,
    };
    use tracing::{error, info};

    #[tokio::test]
    async fn marketplace_test() {
        crate::common::setup();
        let result = Clore::default().marketplace().await;
        info!("{:?}", result);
        assert_eq!(true, result.is_ok());

        if let Ok(cards) = result {
            let server_ids = cards
                .iter()
                // .filter(|item| item.card_number == 2)
                .map(|item| {
                    format!(
                        "{:?} {:?} {}",
                        item.server_id, item.card_type, item.card_number
                    )
                })
                .collect::<Vec<String>>();
            info!("server_ids:{:?}", server_ids);
        } else {
            error!("{:?}", result);
        }
    }

    #[tokio::test]
    async fn wallet_test() {
        crate::common::setup();
        let result = Clore::default().wallet().await;
        assert_eq!(true, result.is_ok());
    }

    #[test]
    fn marketplace_json_test() {
        crate::common::setup();
        let row = String::from(
            r#"{"servers":[{"id":1398,"owner":979,"mrl":1440,"price":{"on_demand":{"bitcoin":0.00029,"CLORE-Blockchain":60},"spot":{"bitcoin":0.00029,"CLORE-Blockchain":50}},"rented":false,"specs":{"mb":"B450 Steel Legend","cpu":"AMD Athlon 200GE with Radeon Vega Graphics","cpus":"2/4","ram":14.91,"disk":"RevuAhnx20900Gx20BIZx 97.5735GB","disk_speed":1754.3859649122808,"gpu":"2x NVIDIA GeForce RTX 3080","gpuram":10,"net":{"up":2.99,"down":4.78,"cc":"KR"},"backend_version":9,"pcie_rev":1,"pcie_width":1,"pl":[320,320]},"reliability":1,"allowed_coins":["bitcoin","CLORE-Blockchain"],"rating":{"avg":5,"cnt":2},"cuda_version":"12.0"},{"id":1397,"owner":979,"mrl":1440,"price":{"on_demand":{"bitcoin":0.00029,"CLORE-Blockchain":60},"spot":{"bitcoin":0.00029,"CLORE-Blockchain":50}},"rented":false,"specs":{"mb":"B450 Steel Legend","cpu":"AMD Athlon 200GE with Radeon Vega Graphics","cpus":"2/4","ram":14.91,"disk":"RevuAhnx20900Gx20BIZx 97.8746GB","disk_speed":1779.359430604982,"gpu":"2x NVIDIA GeForce RTX 3080","gpuram":10,"net":{"up":3.64,"down":1.71,"cc":"KR"},"backend_version":9,"pcie_rev":1,"pcie_width":1,"pl":[320,320]},"reliability":1,"allowed_coins":["bitcoin","CLORE-Blockchain"],"rating":{"avg":5,"cnt":4},"cuda_version":"12.0"},{"id":1393,"owner":929,"mrl":1440,"price":{"on_demand":{"bitcoin":0.00029,"CLORE-Blockchain":60},"spot":{"bitcoin":0.00029,"CLORE-Blockchain":50}},"rented":false,"specs":{"mb":"B450 Steel Legend","cpu":"AMD Athlon 200GE with Radeon Vega Graphics","cpus":"2/4","ram":14.91,"disk":"RevuAhnx20900Gx20BIZx 98.018GB","disk_speed":1742.1602787456447,"gpu":"2x NVIDIA GeForce RTX 3080","gpuram":10,"net":{"up":0.24,"down":3.32,"cc":"KR"},"backend_version":9,"pcie_rev":1,"pcie_width":1,"pl":[320,320]},"reliability":0.9999,"allowed_coins":["bitcoin","CLORE-Blockchain"],"rating":{"avg":5,"cnt":2},"cuda_version":"12.0"},{"id":1229,"owner":775,"mrl":1440,"price":{"on_demand":{"bitcoin":0.0025},"spot":{"bitcoin":0.00017698}},"rented":false,"specs":{"mb":"ROG STRIX Z490-E GAMING","cpu":"Intel(R) Celeron(R) G5905 CPU @ 3.50GHz","cpus":"2/2","ram":15.8085,"disk":" 62.9768GB","disk_speed":2252.252252252252,"gpu":"4x NVIDIA GeForce RTX 3090","gpuram":24,"net":{"up":59.81,"down":39.86,"cc":"KR"},"backend_version":8,"pcie_rev":1,"pcie_width":1,"pl":[350,350,350,350]},"reliability":1,"allowed_coins":["bitcoin"],"rating":{"avg":3.5,"cnt":6},"cuda_version":"12.0"}],"my_servers":[],"code":0}"#,
        );
        let result = serde_json::from_str::<Marketplace>(&row);
        assert_eq!(true, result.is_ok());
    }

    #[tokio::test]
    async fn marketplace_filter_test() {
        crate::common::setup();
        let mut marketplace =
            std::fs::File::open("./market.json").expect("当前目录下./market.json文件不存在！");
        let mut row = String::from("");
        let _ = marketplace.read_to_string(&mut row);
        let result = serde_json::from_str::<Marketplace>(&row);
        assert_eq!(true, result.is_ok());
        let model = result.unwrap();
        let mut cards: Vec<Card> = model.filter();
        cards = cards
            .into_iter()
            .filter(|item| item.card_number == 2)
            .map(|item| item)
            .collect::<Vec<Card>>();
        info!("server_ids:{:?}", cards);
        if cards.len() > 0 {
            let resent_server_id = cards.get(0).unwrap();
            info!("resent_server_id:{:?}", resent_server_id);
        }

        assert_eq!(std::any::TypeId::of::<Vec<Card>>(), cards.type_id())
    }

    #[tokio::test]
    async fn create_order_test() {
        crate::common::setup();
        panic!("请更改id测试！！");
        let resent_ids = [16296, 23859];
        let market = Clore::default().marketplace().await;
        if let Ok(cards) = market {
            let cards = cards
                .into_iter()
                // .filter(|item| item.card_number == 2)
                .collect::<Vec<_>>();
            info!("server_ids:{:?}", cards);
            if cards.len() > 0 {
                let resent_server = cards.get(0).unwrap();
                let address = Vec::<String>::new();
                if resent_server.card_number > address.len() as u32 {
                    panic!("所租显卡无法进行地址分配");
                }
                info!("resent_server_id:{:?}", resent_server);
                let result = Clore::default().create_order(&resent_server, address).await;
                info!("create_order_test:{:?}", result);
                assert_eq!(true, result.is_ok())
            }
        }
    }

    #[tokio::test]
    async fn create_order_web_api_test() {
        crate::common::setup();
        // panic!("请更改id测试！！");
        let resent_ids = [22566];
        let address = Vec::<String>::new();
        let clore = Clore::default();
        let mut cards = clore.marketplace().await.unwrap();
        cards = cards
            .iter()
            .filter(|card| resent_ids.contains(&card.server_id))
            .map(|card| card.clone())
            .collect::<Vec<Card>>();
        for card in cards.iter() {
            if card.card_number > address.len() as u32 {
                panic!("所租显卡无法进行地址分配");
            }
            let addr = address[0..(card.card_number as usize)].to_vec();
            let _ = clore.create_order_web_api(card, addr).await;
        }
    }

    #[tokio::test]
    async fn my_orders_test() {
        crate::common::setup();
        let my_orders_result = Clore::default().my_orders().await;
        assert!(my_orders_result.is_ok());
        if my_orders_result.is_ok() {
            let mut my_orders = my_orders_result.unwrap();
            // Ssh::try_run_command_remote(&mut my_orders).await;
        }
    }

    #[test]
    fn import_block_server_ids_test() {
        crate::common::setup();
        Clore::import_block_server_ids();
    }

    #[test]
    fn append_block_server_id_test() {
        crate::common::setup();
        Clore::append_block_server_id(77777);
    }

    #[tokio::test]
    async fn cancel_order_test() {
        crate::common::setup();
        let orderid = 259841;
        let result = Clore::default().cancel_order(orderid).await;
        info!("{:?}", result);
        assert_eq!(true, result.is_ok())
    }

    #[tokio::test]
    async fn cancel_order_web_api_test() {
        crate::common::setup();
        let orderid = 274087;
        Clore::default().cancel_order_web_api(orderid).await;
    }
}
