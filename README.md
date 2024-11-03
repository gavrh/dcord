# dcord-rs

[![Documentation](https://img.shields.io/badge/Docs.rs-gray?logo="docs.rs"&logoColor="yellow")](https://docs.rs/dcord)
![Size](https://img.shields.io/github/languages/code-size/gavrh/dcord?label="Size"&color="green")

[![License](https://img.shields.io/github/license/gavrh/dcord?label="License"&color="AA55AA")](https://github.com/gavrh/dcord/blob/master/LICENSE)
[![Discord](https://img.shields.io/discord/81384788765712384?label="Discord%20API"&logo="discord"&logoColor="7289da"&color="7289da")](https://discord.com/invite/discord-api)

[comment]:<img align="right" src="https://i.imgur.com/QizpY58.png" width="300" />

> [!CAUTION]
>
> This library is in early stage development and is not ready for any sort of use or production build.

## Getting Started

### Github


### Cargo
```
cargo add dcord
```

#### `Cargo.toml` File
```toml
[dependencies]
dcord = "0.0.3"
tokio = { version = "1.37.0", features = ["macros", "rt-multi-thread"] }

# if you want to alias the package
dcord = { package = "discord", version = "0.0.3" }
```

### Starter Code

