use crate::config::settings::Settings;
use config::Config;
use mongodb::{bson::doc, options::ClientOptions, Client};
use std::sync::Arc;

pub async fn init_mongo() -> Arc<Client> {
    // 加载配置
    let settings = Config::builder()
        .add_source(config::File::with_name("config/default"))
        .build()
        .unwrap();
    let settings: Settings = settings.try_deserialize().unwrap();

    // 使用配置初始化 MongoDB 客户端
    let client_options = ClientOptions::parse(&settings.database.uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let client = Arc::new(client);

    // 使用 ping 命令检查连接状态
    let db = client.database("admin");
    match db.run_command(doc! {"ping": 1}).await {
        Ok(_) => println!("Connected to MongoDB at {}", settings.database.uri),
        Err(e) => {
            eprintln!("Failed to connect to MongoDB: {}", e);
            std::process::exit(1);
        }
    }

    client
}
