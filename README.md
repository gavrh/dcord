# disgord
![GitHub Release](https://img.shields.io/github/v/release/grhx/disgord?label=pkg%20version) [![Go Reference](https://pkg.go.dev/badge/github.com/grhx/disgord.svg)](https://pkg.go.dev/github.com/grhx/disgord) ![GitHub go.mod Go version](https://img.shields.io/github/go-mod/go-version/grhx/disgord?logo=go&label=go.mod&color=08AFD8) [![Discord Gophers](https://img.shields.io/badge/Discord%20Gophers-%23disgord-blue.svg)](https://discord.gg/golang) [![Discord API](https://img.shields.io/badge/Discord%20API-%23go_disgord-blue.svg)](https://discord.com/invite/discord-api)


<img align="right" src="https://avatars.githubusercontent.com/u/85959578?s=280&v=4" width="400" />


Disgord is your ultimate solution for seamless integration with the Discord chat client API, offering robust low-level bindings for effortless interaction. Designed to empower developers with comprehensive access, Disgord provides unparalleled support for Discord's API endpoints, WebSocket interface, and voice functionality.With its nearly complete coverage of all Discord API features, Disgord unlocks limitless possibilities for building dynamic and immersive experiences within the Discord ecosystem. Whether you're crafting bots, managing communities, or orchestrating engaging interactions, Disgord empowers you to unleash your creativity with ease and efficiency. Harness the full potential of Discord integration effortlessly with Disgord.


## Getting Started
### Installing
###### If you haven't set up a working Go environment yet, please refer to [this page]() before proceeding.
`go get` *will retrieve the most recent release from the master branch.*
```cmd
go get github.com/grhx/disgord
```
### Usage
Import the package into your project.
```go
import "github.com/grhx/disgord"
```
Create a fresh Discord client enabling access to a multitude of Discord API functionalities and the ability to define callback functions for Discord events. Initiate the session by invoking the `Client.Login()` function.
```go
// initialize new discord client
client, err := discord.New(intents int, partials int, api int, shards int)


// OnReady callback
client.OnReady(func(){
    fmt.Printf("%s is online...", client.User.Username)
})
... // other callbacks, error checks, and whatever else you want

// start client session
client.Login(token string)
```

## Documentation
documentation coming soon... most code snippets are currently future planned.

## Examples
examples will be coming soon...



<div align="center">
    <br/>
    <br/>
    <br/>
    <h3>Made with ❤️</h3>
    by: <a href="https://github.com/grhx" target="_blank">github.com/grhx</a> ~> <a href="https://grh.dev" target="_blank">grh.dev</a>
    <br/>
    <br/>
    <img align="center" width=100 src="https://user-images.githubusercontent.com/79518089/141609256-ddcafafa-dca0-4cc3-b203-008e441ae2a2.gif"/>
</div>
