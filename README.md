# This is a joke website. Rust + Rocket
Run:
1) edit .env - specify your PostgreSQL username, password, host and DB

2) install diesel cli

3)
``` bash
cd joke-website
diesel setup
diesel migration run
```
4) fill db tables users and adjectives

5)
``` bash
cargo +nightly-2018-07-24 run
```
[Here](http://130.193.58.61) you can see it in action. CAUTION! Russian language!
