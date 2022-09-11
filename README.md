# A blazingly (🔥) fast (🚀) Rust http snake game server.

# Overview

Server creates 2 endpoints: 
- **GET** [/snake](http:localhost:3721/snake) which returns game board
- **POST** /snake/{direction} which puts direction in a buffer, from which next snake direction will be chosen randomly.
Direction can be one of [/up, /down, /left or /right].

In case of direction being opposite to the current one (e.g. left and right), server will return error response.
Game is updated every second by default.

## 0. Prerequisites

Have Rust installed

## 1. Setup

Clone the repo
```shell
git clone https://github.com/szaramewa/http_snake
```

Build project
```shell
cargo build --release
```

## 2. Run server
```shell
cargo run -r --bin server
```

Server listens on port 3721, so make sure nothing else is using it.
Game is printed on the console, each time it is updated.

## 3. Run simple client
```shell
cargo run -r --bin client
```

This allows you to move snake using WSAD keys.


## 4. Run spam client
```shell
cargo run -r --bin spammer
```

This client sends around 350 requests per game tick on my machine.

## 5. Shutdown

When you are done shut down server/clients with Ctrl - c.

## 6. Run tests

To run cargo tests use
```shell
cargo test
```
To test api use postman or curl commands.

```shell
curl -X POST http://127.0.0.1:3721/snake/up
```

```shell
curl -X POST http://127.0.0.1:3721/snake/down
```

```shell
curl -X POST http://127.0.0.1:3721/snake/left
```

```shell
curl -X POST http://127.0.0.1:3721/snake/right
```


```shell
curl http://127.0.0.1:3721/snake
```


















