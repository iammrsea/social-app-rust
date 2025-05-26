use mongodb::{Client, options::ClientOptions};
use std::env;
use tokio::sync::OnceCell;

static MONGO_CLIENT: OnceCell<Client> = OnceCell::const_new();

pub async fn setup_test_mongo() -> &'static Client {
    MONGO_CLIENT
        .get_or_init(|| async {
            let mongo_uri = env::var("MONGO_URI").expect("MONGO_URI environment variable not set");
            println!("mongo_uri: {}", mongo_uri);
            let options = ClientOptions::parse(&mongo_uri)
                .await
                .expect("Failed to parse MongoDB options");
            println!("mongo_options: {:#?}", options);
            let client = Client::with_options(options).expect("Failed to create MongoDB client");
            client
        })
        .await
}

// async fn wait_until_ready(client: &Client) {
//     use std::time::Duration;
//     use tokio::time::sleep;

//     for _ in 0..10 {
//         if client.list_database_names().await.is_ok() {
//             return;
//         }
//         sleep(Duration::from_millis(300)).await;
//     }

//     panic!("MongoDB did not become ready in time");
// }
