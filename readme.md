# Carolus

[![Build Status](https://travis-ci.org/carolustv/carolus-server.svg?branch=master)](https://travis-ci.org/carolustv/carolus-server)

Quick start:

```bash
export CAROLUS_MOVIES_PATH="/my/movies/path"
export CAROLUS_TV_PATH="/my/tv/path"
cargo run &
curl http://localhost:8080/api/movies
```

## Build Docker Image

```bash
cargo build --release
docker build -t carolustv/carolus .
```

## TLS support

A quick way to get started with using tls is included in the repo (taken
from [Rocket examples](https://github.com/SergioBenitez/Rocket/tree/master/examples/tls)).
Run the following:

```bash
(cd private && bash ./gen_cert.sh)
export ROCKET_TLS={certs="private/ca_cert.pem",key="private/ca_key.pem"}
cargo run --feature=tls &
```
