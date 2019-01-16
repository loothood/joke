# This is a joke website. Rust + Rocket
Run:
edit .env - specify your PostgreSQL username, password, host and DB
install diesel cli
run:

1)
``` bash
cd joke
diesel setup
diesel migration run
```
2) fill db tables users and adjectives

3)
``` bash
cargo +nightly-2018-07-24 run
```
[Here](http://130.193.58.61) you can see it in action. CAUTION! Russian language!
