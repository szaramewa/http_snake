# A blazingly 🔥 fast 🚀 rust http snake game server.

1. ## Setup

Clone the repo
```shell
git clone https://github.com/szaramewa/http_snake
```

Build project
```shell
cargo build --release
```

2. ## Run server
```shell
cargo run -r --bin server
```

Server listens on port 3721, so make sure nothing else is using it.
Game is printed on the console, each time it is updated.

3. ## Run simple client
```shell
cargo run -r --bin client
```

This allows you to move snake using WSAD


4. ## Run spam client
```shell
cargo run -r --bin spammer
```

This client sends around 350 requests per game tick on my machine


5. ## Run tests
```shell
cargo test
```


