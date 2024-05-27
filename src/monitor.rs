use lazy_static::lazy_static;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

use crate::monitor::log::read_log_file;

use self::log::Logs;
use self::nvidia::GeForces;
pub mod log;
pub mod nvidia;

lazy_static! {
    pub static ref MONITOR: Arc<Mutex<Monitor>> = { Arc::new(Mutex::new(Monitor::new())) };
}
pub struct Monitor {
    server_id: Option<u32>,
    nvidias: GeForces,
    logs: Logs,
    upload_log: HashMap<String, Vec<String>>,
}

impl Monitor {
    fn new() -> Monitor {
        Monitor {
            server_id: Monitor::get_server_id(),
            nvidias: GeForces::new(),
            logs: Logs::new(),
            upload_log: HashMap::<String, Vec<String>>::new(),
        }
    }

    fn get_server_id() -> Option<u32> {
        let server_id = std::env::var("server_id")
            .map_err(|e| e.to_string())
            .and_then(|server_id| server_id.parse::<u32>().map_err(|e| e.to_string()))
            .ok();
        server_id
    }

    pub async fn check(&mut self) {
        
        self.logs.iter_log_files().await;

        for log  in self.logs.iter_mut() {
            info!("{:?}",log);
            if !log.spawn {
                log.spawn = true;
                tokio::spawn(read_log_file(log.clone()));
            }
            
            info!("{:?}",log);
        }
    }

    pub async fn upload(&self) {
        info!("上报数据:\n");
        // let client = ClientBuilder::new().build().unwrap();
        // let result = client
        //     .post(LOG_COLLECT_API)
        //     .body(body.to_string())
        //     .send()
        //     .await;
        // if result.is_err() {
        //     error!("上报数据失败:{:?}", result);
        // } else {
        //     info!("上报成功:{:?}", result)
        // }
    }

    pub async fn mining(&self, address: &str) -> Result<bool, String> {
        //测试地址
        let dir = std::env::current_dir().unwrap().join("run.sh");
        let result = std::process::Command::new("bash")
            .args([dir.to_str().unwrap(), &address])
            .output();
        match result {
            Ok(output) => {
                if output.status.success() {
                    Ok(true)
                } else {
                    Err(format!("运行退出异常：{:?}", output.status.code()))
                }
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

pub async fn monitor() {
    loop {
        let monitor = Arc::clone(&MONITOR);
        let mut locked = monitor.lock().await;
        (*locked).check().await;

        drop(locked);
    }
}
