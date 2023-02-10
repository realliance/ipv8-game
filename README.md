# IPv8
*Game Service*

[![codecov](https://codecov.io/github/realliance/ipv8-game/branch/main/graph/badge.svg?token=N0NY2XR28V)](https://codecov.io/github/realliance/ipv8-game)

## Getting Started

### Install dependencies
*On Mac use `brew bundle`*

- `capnp`
- `postgresql-dev` (whatever can give you `libpq`)

### Note for Mac

You will additionally need to link the installed `libpq` crate via `brew link --force libpq`

```
# Bring up the dev services
docker-compose up -d

# Generate a default properties file
cargo run -- gen-config

# Start the server
cargo run
```
