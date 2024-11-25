use crate::config::settings::Settings;
use mongodb::{bson::doc, options::ClientOptions, Client};
use std::sync::Arc;

pub async fn init_mongo(settings: &Settings) -> Arc<Client> {
    let uri = format!(
        "mongodb://{}:{}@{}:{}/{}",
        settings.mongo.username,
        settings.mongo.password,
        settings.mongo.host,
        settings.mongo.port,
        settings.mongo.database
    );

    // 使用配置初始化 MongoDB 客户端
    let client_options = ClientOptions::parse(&uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let client = Arc::new(client);

    // 使用 ping 命令检查连接状态
    let db = client.database("admin");
    match db.run_command(doc! {"ping": 1}).await {
        Ok(_) => {
            let safe_uri = format!(
                "mongodb://{}:{}@{}:{}/{}",
                settings.mongo.username,
                "****",
                settings.mongo.host,
                settings.mongo.port,
                settings.mongo.database
            );
            println!("Connected to MongoDB at {}", safe_uri);
        }
        Err(e) => {
            eprintln!("Failed to connect to MongoDB: {}", e);
            std::process::exit(1);
        }
    }

    client
}
