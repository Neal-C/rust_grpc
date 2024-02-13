# A Rust Playground
## Chill, this is not meant for production. it is not "prod ready"

### Technical & design choices

Every design and tech decision were taken based in order of importance by: what I want to experiment/discover, what I like, what's been recommended to me by the Rust community

### Choices

to gain time, and give a minimum viable product, I overlooked several things:

- database initialization, database best practices
- security (errors returned to the client, etc...)
- proper typing 
- error and exception handling
- clean architecture patterns
- validating and/or escaping user input

### Tech Stack

- Rust
- tonic gRPC
- PostgreSQL

## Specific tools

- special IDE configurations for gRPC intellisense 

## Requirements

- an environment with Rust installed
- gRPC compilers

### Instructions

- install dependencies: Rust, gRPC compilers
- bring your postgresql database
- set environment variables ( see .env.example)

```shell
cargo run --bin quote-server
```

A hardcoded client request to the server
```shell
cargo run --bin quote-client
```

