use std::env;

use base64::Engine;
use dotenv::dotenv;
use e6kiro::E621Posts;
use reqwest::header::{HeaderValue, AUTHORIZATION, USER_AGENT};
use serenity::all::CreateMessage;
use serenity::model::channel::Message;
use serenity::prelude::*;
use serenity::{all::CreateAttachment, async_trait};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {why:?}");
            }
        }

        if msg.content.starts_with("!e6") {
            let typing = msg.channel_id.start_typing(&ctx.http);
            let content = msg.content.replace("!e6 ", "");

            let mut tags = content
                .split(",")
                .map(|s| s.to_owned())
                .collect::<Vec<String>>();

            let mut last_tag = tags.last().ok_or("No tags, somehow").unwrap().to_owned();

            last_tag = last_tag
                .split(" ")
                .collect::<Vec<&str>>()
                .first()
                .ok_or("No first element, somehow")
                .unwrap()
                .to_string();

            if let Some(last) = tags.last_mut() {
                *last = last_tag;
            }

            if content.contains("--safe") {
                tags.push("rating:safe".to_owned())
            } else {
                tags.push("rating:explicit".to_owned())
            }

            let mut quantity = 1;

            if let Some(arg) = content.split(" ").nth(1) {
                match arg.parse::<u32>() {
                    Ok(num) => {
                        println!("{num:?}");
                        quantity = num;
                    }
                    Err(why) => {
                        println!("Error: possibly not a number, {why:?}")
                    }
                }
            }

            let res = E621::new(tags, ctx, msg, std::cmp::min(quantity, 10)).await;

            typing.stop();

            res.unwrap();
        }
    }
}

const E6_URL: &str = "https://e621.net/posts.json";

struct E621;

impl E621 {
    async fn new(
        tags: Vec<String>,
        ctx: Context,
        msg: Message,
        quantity: u32,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tags = tags.join("+");

        let client = reqwest::Client::new();

        let mut headers = reqwest::header::HeaderMap::new();

        headers.append(
            USER_AGENT,
            HeaderValue::from_static("e6kiro / made by Kalka"),
        );

        let token = env::var("E621_TOKEN").expect("Expected e621 token");

        let encoding = base64::engine::general_purpose::STANDARD.encode(format!("kalka:{}", token));

        let t = format!("Basic {}", encoding);
        let header_value = HeaderValue::from_str(&t).expect("Invalid header value");

        headers.append(AUTHORIZATION, header_value);

        println!("{tags} {quantity}");

        let res = client
            .get(format!(
                "{E6_URL}?limit={quantity}&tags=order:random+-female+-intersex+{tags}"
            ))
            .headers(headers)
            .send()
            .await?;

        let data = res.json::<E621Posts>().await?;

        if data.posts.is_empty() {
            if let Err(why) = msg.channel_id.say(&ctx.http, "No post found!").await {
                println!("Error sending message: {why:?}");
            }

            return Err("Posts is empty".into());
        }

        let mut files = vec![];

        for post in data.posts.iter() {
            let mut file = CreateAttachment::url(&ctx.http, &post.file.url).await?;

            file.filename = format!("SPOILER_{}", file.filename);

            files.push(file)
        }

        let sent_msg = msg
            .channel_id
            .send_files(&ctx.http, files, CreateMessage::new());

        if let Err(why) = sent_msg.await {
            println!("Could not send: {why:?}");

            // Send as a link instead

            let post = data
                .posts
                .first()
                .ok_or("Somehow, the post was not found.")?;

            let e6_url = format!("https://e621.net/posts/{}", post.id);

            msg.channel_id
                .say(&ctx.http, format!("||{}||", e6_url))
                .await?;
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    let user = client.http.get_current_user().await?;

    //let conf = viuer::run();

    println!("Logged in as {}", user.name);

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }

    Ok(())
}
