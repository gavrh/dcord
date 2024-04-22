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

### With Github
coming soon...

### With Cargo
```
cargo add dcrs
```

### Simple Starter Code
```rs
use dcrs::{
    Client,
    event::EventHandler
    message::Message
}

// create a handler struct and implement 'EventHandler' trait functions
// all functions have a referenced client param to use context and cache
struct Handler;
impl<'a> EventHandler<'a> for Handler {

    // ready event
    // @params -> ref to client
    fn ready(client: &'a Client<'a>) {
        println!("{:?} is now online.", client.user.username);
    }

    // message create event
    // @params -> ref to client, ref to message
    fn message_create(client: &'a Client<'a>, message: &'a Message<'a>) {
        println!("{:?}", message.content);
    }

} 

fn main() {
    // build new client
    let client = Client::new();
    // login and start session with client token
    // this uses the 'dotenv' package
    // TOKEN is stored in a '.env' file
    client.login(dotenv::var("TOKEN").unwrap());
}
```