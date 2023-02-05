# rekt

## What is it?

Rust CRUD app with basic auth written over the weekend.

## Requirements

* Rust
* Cargo
* Docker

## Running

To start database inside docker container:

```shell
docker compose up -d
```

To run API server:

```shell
cargo run
```

## What's included?

* Source code (kinda obvious).
* Postman collection inside docs folder. Should work instead of documentation for now.

## TODO

* Better logging.
* Proper documentation, maybe with swagger.
* Frontend.