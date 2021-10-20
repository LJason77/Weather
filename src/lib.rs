#![deny(clippy::pedantic)]
#![allow(clippy::non_ascii_literal)]

use mongodb::{Client, Database};
use serde::{Deserialize, Serialize};

mod simulator;

#[derive(Deserialize, Serialize, Debug)]
pub struct Weather<'a> {
    /// 设备
    pub device: &'a str,
    /// 时间：毫秒
    pub ts: i64,
    /// 温度：摄氏度
    pub temp: f32,
    /// 湿度：%
    pub humidity: u8,
    /// 风向：度
    #[serde(rename = "windDirection")]
    pub wind_direction: i16,
    /// 风速：m/s
    #[serde(rename = "windSpeed")]
    pub wind_speed: u16,
}

/// 初始化 mongodb 数据库
pub async fn init_mongo(name: &str) -> Database {
    let database_url = format!("mongodb://weather:weather@192.168.50.176:27017/{}", name);
    let client = Client::with_uri_str(&database_url)
        .await
        .expect("连接数据库失败：");
    client.database(name)
}

#[cfg(test)]
mod tests {
    use crate::simulator::execute;

    #[tokio::test]
    async fn simulator() {
        execute().await;
    }
}
