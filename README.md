# This is a joke website
Run:
edit .env - specify your PostgreSQL username, password, host and DB
install diesel cli
run:
``` rust
cd joke
diesel setup
diesel migration run
```
fill db tables users and adjectives
``` rust
cargo +nightly-2018-07-24 run
```
