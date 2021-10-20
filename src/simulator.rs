use rand::Rng;

use crate::{init_mongo, Weather};

/// 插入虚拟数据
pub async fn execute() {
    let db = init_mongo("weather").await;
    let weather = db.collection::<Weather>("weather");
    // 100 个点位
    let mut device_list = Vec::<String>::with_capacity(100);
    for i in 0..100 {
        device_list.push(format!("code_{}", i))
    }

    let mut rng = rand::thread_rng();
    // 每个点位一万条数据
    for _ in 0..10000 {
        let mut data = Vec::<Weather>::with_capacity(10000);
        for device in &device_list {
            let weather = Weather {
                device,
                ts: chrono::Local::now().timestamp_millis(),
                temp: rng.gen_range(0.0..38.0),
                humidity: rng.gen_range(0..100),
                wind_direction: rng.gen_range(0..360),
                wind_speed: rng.gen_range(0..30),
            };
            data.push(weather);
        }
        weather.insert_many(data, None).await.unwrap();
    }
}
