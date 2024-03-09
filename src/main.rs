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
    game_name: String,
    viewer_count: i32,
    started_at: String,
    language: String,
    thumbnail_url: String,
    tags: Vec<String>,
    // tag_ids: Vec<String>,
    // is_mature: bool,
    // type: String,
    // id: String,
    // user_login: String,
    // user_login_id: String,
    // user_login_name: String,
    // user_login_type: String,
    // user_login_email: String,
    // user_login_description: String,
    // user_login_offline_image_url: String,
    // user_login_profile_image_url: String,
    // user_login_view_count: i32,
    // user_login_email_verified: bool,
    // user_login_created_at: String,
    // user_login_updated_at: String,
    // user_login_broadcaster_type: String,
    // user_login_stream_key: String,
    // user_login_is_verified: bool,
    // user_login_is_partner: bool,
    // user_login_is_affiliate: bool,
    // user_login_is_transcoding_enabled: bool,
    // user_login_is_vod_recording_enabled: bool,

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let access_token = env::var("TWITCH_ACCESS_TOKEN")
        .expect("TWITCH_ACCESS_TOKEN não definido no arquivo .env");
    let client_id = env::var("TWITCH_CLIENT_ID")
        .expect("TWITCH_CLIENT_ID não definido no arquivo .env");
    let category_name = "1469308723"; // ID da categoria desejada
    let pagination = 100;
    let language = "pt"; // Idioma desejado
    let status = "live"; // Status desejado "live" ou "all
    let client = reqwest::Client::new();
    let url = format!("https://api.twitch.tv/helix/streams?game_id={}&first={}&language={}&type={}", category_name , pagination, language, status);

    let response = client.get(&url)
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Client-Id", client_id)
        .send()
        .await?;

    if response.status().is_success() {
        let body = response.text().await?;
        let twitch_response: TwitchResponse = serde_json::from_str(&body)?;

        println!("Streams na categoria '{}':", category_name);
        let mut user_count = 0; // Contador de resultados de usuário
        for stream in twitch_response.data {
            user_count += 1; // Incrementa o contador de resultados de usuário
            println!("{}: Streamer: {}, Título da Live: {}, Categoria: {}, Viewers: {}, Início: {}, Tags: {:?}, Thumb: {}", user_count, stream.user_name, stream.title, stream.game_name, stream.viewer_count, stream.started_at, stream.tags, stream.thumbnail_url );
        }
        println!("Total de resultados de usuário: {}", user_count); // Imprime o total de resultados de usuário
    } else {
        println!("Erro: {}", response.status());
        if let Ok(body) = response.text().await {
            println!("Detalhes do erro: {:?}", body);
        }
    }

    Ok(())
}
