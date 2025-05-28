# Docker puller

## Installation
Build using `cargo build --release`,
copy the executable from `target/release/docker-puller` to anywhere you want to use this.

## Usage
`docker-puller cli` for local cli usage:
```
Usage: docker-puller cli [OPTIONS] <SERVICE_NAME>

Arguments:
  <SERVICE_NAME>  

Options:
  -p, --path <PATH>  
  -h, --help         Print help
```

`docker-puller server` to start HTTP server:

Provide the environment variable `SECRET` with the secret token that clients should provide.
```
Usage: docker-puller server [OPTIONS]

Options:
  -p, --port <PORT>  
  -h, --help         Print help
```
Clients can make a get request to the server with the following paramters:
`?service=<SERVICE_NAME>&path=<PATH>&token=<TOKEN>`, where `path` is optional.
