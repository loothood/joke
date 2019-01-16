# This is a joke website. Rust + Rocket
Run:
edit .env - specify your PostgreSQL username, password, host and DB
install diesel cli
run:
``` bash
cd joke
diesel setup
diesel migration run
```
fill db tables users and adjectives
``` bash
cargo +nightly-2018-07-24 run
```
[Here](http://130.193.58.61) you can see it in action. CAUTION! Russian language!
