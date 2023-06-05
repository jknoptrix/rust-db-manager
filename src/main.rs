mod config;
mod database;
use config::config::{DatabaseConfig, DatabaseConfigImpl};

#[tokio::main]
async fn main() {
    let mut config = DatabaseConfigImpl::new();

    let db = config.db();
    let handle1 = config.take_handle1().unwrap();
    let handle2 = config.take_handle2().unwrap();

    handle1.await.unwrap();
    handle2.await.unwrap();

    let data = db.data.lock().unwrap();
    println!("{:?}", *data);
}
