# dcrs

> made with <3

[![Documentation](https://img.shields.io/badge/Docs.rs-gray?logo="docs.rs"&logoColor="yellow")](https://docs.rs/dcrs)
[![Version](https://img.shields.io/crates/v/dcrs?label="Crates.io"&color="orange"&logo="rust"&logoColor="orange")](https://crates.io/crates/dcrs)
![Size](https://img.shields.io/github/languages/code-size/grhx/dcrs?label="Size"&color="green")
[![Downloads](https://img.shields.io/crates/d/dcrs?label="Downloads"&color="blue")](https://crates.io/crates/dcrs)

[![License](https://img.shields.io/github/license/grhx/dcrs?label="License"&color="AA55AA")](https://github.com/grhx/dcrs/blob/master/LICENSE)
[![Discord](https://img.shields.io/discord/81384788765712384?label="Discord%20API"&logo="discord"&logoColor="7289da"&color="7289da")](https://discord.com/invite/discord-api)

[comment]:<img align="right" src="https://i.imgur.com/QizpY58.png" width="300" />

Introducing **dcrs**, your ultimate solution for seamless integration with the Discord chat client API. This Rust library offers blazingly fast performance and comprehensive functionality, making interaction with Discord's API effortless. Designed to empower developers with robust low-level bindings, **dcrs** provides unparalleled support for Discord's API endpoints, WebSocket interface, and voice functionality.

With its focus on speed and simplicity, **dcrs** unlocks limitless possibilities for building dynamic and immersive experiences within the Discord ecosystem. Whether you're crafting bots, managing communities, or orchestrating engaging interactions, **dcrs** empowers you to unleash your creativity with ease and efficiency.

Harness the full potential of Discord integration effortlessly with **dcrs**.

#

> [!CAUTION]
>
> This library is in early stage development and is not ready for any sort of use or production build, most features are incomplete or missing.

## Getting Started

### Github
setup coming soon...

### Cargo
```
cargo add dcrs
```

#### `Cargo.toml` File
```toml
[dependencies]
dcrs = "0.0.3"
tokio = { version = "1.37.0", features = ["macros", "rt-multi-thread"] }
```

### Starter Code
```rs
use dcrs::{
    async_trait,
    client::{
        Client,
        EventHandler,
        Context
    },
    obj::Message,
    gateway::GatewayIntent
};

struct Handler;
#[async_trait]
impl EventHandler for Handler {

    async fn ready(&self, ctx: Context) {
        println!("{} is online...", ctx.user.name);
    }

    async fn message_create(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel.send(&ctx.http, "Pong!").await {
                println!("Err sending message: {why:?}");
            }
        }
    }

}

#[tokio::main]
async fn main() {

    // Grab token from environment (best practice)
    let token = dotenv::var("TOKEN").unwrap():
    // Set gateway intents, which will determine events received.
    let intents = vec![GatewayIntent::All];

    // Create new Client
    let mut client = Client::builder(token, intents)
    .event_handler(Handler) // bind event handler
    .verbose_mode()         // verbose logging
    .await.expect("Err creating client.");

    if let Err(why) = client.start().await {
        println!("{why:?}");
    }

}
```