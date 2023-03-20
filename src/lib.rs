use dotenv::dotenv;
use reqwest::header::HeaderMap;
use serde::{Deserialize, Serialize};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::{collections::HashMap, env};
use tracing::{error, info};

struct Bot;

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        // bot or DM or 実験所限定
        if msg.author.bot || msg.is_private() || msg.channel_id != 1085572570595737701 {
            return;
        }
        if msg.content == "!hello" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        } else if msg.content == "!rust" {
            if let Err(e) = msg.channel_id.say(&ctx.http, "change the world.").await {
                error!("Error sending message: {:?}", e);
            }
        } else if msg.content.starts_with("!gpt ") {
            let message = msg.content.replace("!gpt ", "");
            info!("called ChatGPT '{}'", message);

            let res_ms = call_chat_gpt_api(message).await;
            if let Err(e) = msg.channel_id.say(&ctx.http, res_ms).await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

pub async fn call_chat_gpt_api(message: String) -> String {
    let api_key = env::var("CHATGPT_API_KEY").expect("'CHATGPT_API_KEY' was not found");

    let openai_url = "https://api.openai.com/v1/chat/completions";

    // make headers
    let mut headers = HeaderMap::new();

    headers.insert(
        "Authorization",
        format!("Bearer {}", api_key).parse().unwrap(),
    );
    headers.insert("Content-Type", "application/json".parse().unwrap());

    let mut cont: HashMap<String, String> = HashMap::new();
    cont.insert("role".to_string(), "user".to_string());
    cont.insert("content".to_string(), message);
    let messages = vec![cont];

    // make body
    let request_body = ChatRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages,
    };

    info!("{:?}", serde_json::to_string(&request_body));

    let client = reqwest::Client::new();
    let req = client
        .post(openai_url)
        .headers(headers)
        .json(&request_body)
        .send()
        .await
        .expect("failed to get response")
        .json::<ChatResponse>()
        .await
        .expect("failed to convert json");

    // let gpt_response = match req {
    //     Ok(res) => res,
    //     Err(err) => {
    //         panic!("{:?}", err);
    //     }
    // };

    // if !gpt_response.status().is_success() {
    //     error!("Fail to post...");
    // }

    // let j = gpt_response.json();
    let res_message = req.choices[0]
        .message
        .get("content")
        .expect("faild to get content.")
        .to_string();

    let used_tokens = req.usage.total_tokens;

    info!("used token: {}", used_tokens);

    res_message
}

#[shuttle_service::main]
async fn serenity() -> shuttle_service::ShuttleSerenity {
    // read env
    dotenv().ok();

    let token = env::var("DISCORD_TOKEN").expect("'DISCORD_TOKEN' was not found");

    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let client = Client::builder(&token, intents)
        .event_handler(Bot)
        .await
        .expect("Err creating client");

    Ok(client)
}

// === structs ===
#[derive(Serialize, Deserialize)]
struct ChatRequest {
    model: String,
    messages: Vec<HashMap<String, String>>,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct ChatResponse {
    id: String,
    object: String,
    created: u32,
    choices: Vec<Choices>,
    usage: Usage,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Choices {
    index: u32,
    message: HashMap<String, String>,
    finish_reason: String,
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Usage {
    prompt_tokens: u32,
    completion_tokens: u32,
    total_tokens: u32,
}
