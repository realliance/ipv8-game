# IPv8
*Game Service*

[![codecov](https://codecov.io/github/realliance/ipv8-game/branch/main/graph/badge.svg?token=N0NY2XR28V)](https://codecov.io/github/realliance/ipv8-game)

## Getting Started

### Install dependencies
*On Mac use `brew bundle`*

- `capnp`
- `postgresql-dev` (whatever can give you `libpq`)
- `just`
- Container runtime and Docker Compose compatible tool

### Note for Mac

You will additionally need to link the installed `libpq` crate via `brew link --force libpq`

```
# Bring up the dev services
kubectl apply -k manifests/overlays/development

# Generate env file
just

# Generate a default properties file
cargo run -- gen-config

# (In a new terminal) Port forward the db locally
kubectl -n ipv8-dev port-forward services/psql 5432

# Start the server
cargo run
```
