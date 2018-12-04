# joke
Run:
edit .env - specify your PostgreSQL username, password, host and DB
install diesel cli
run:
```
cd joke
diesel setup
diesel migration run
cargo +nightly-2018-07-24 run
```
