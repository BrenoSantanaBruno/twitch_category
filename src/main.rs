use serde::{Deserialize, Serialize};
use reqwest;
use dotenv::dotenv;
use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct TwitchResponse {
    data: Vec<TwitchStream>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TwitchStream {
    user_id: String,
    user_name: String,
    game_id: String,
    title: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let access_token = env::var("TWITCH_ACCESS_TOKEN")
        .expect("TWITCH_ACCESS_TOKEN não definido no arquivo .env");
    let client_id = env::var("TWITCH_CLIENT_ID")
        .expect("TWITCH_CLIENT_ID não definido no arquivo .env");
    // let category_name = "33214"; // Substitua "NomeDaCategoria" pela categoria desejada | Fortnite: 33214
    let category_name = "21779";
    let client = reqwest::Client::new();
    let url = format!("https://api.twitch.tv/helix/streams?game_id={}&first=2&language=pt", category_name);


    // let url = format!("https://api.twitch.tv/helix/streams?game_id=21779?first=100");


    let response = client.get(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Client-Id", client_id)
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let twitch_response: TwitchResponse = serde_json::from_str(&body)?;

        println!("Streams na categoria '{}':", category_name); // #TODO Converter ID da categoria para nome
        for stream in twitch_response.data {
            println!("Usuário: {}, Título: {}", stream.user_name, stream.title); // #TODO adicionar paginação* para mais resultados
        }
    } else {
        println!("Erro: {}", response.status());
        if let Ok(body) = response.text().await {
            println!("Detalhes do erro: {:?}", body);
        }
    }

    Ok(())
}
