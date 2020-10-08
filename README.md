# DigitalOcean [App Platform](https://www.digitalocean.com/products/app-platform/): Rust

Blog: https://pretired.dazwilkin.com/posts/201008/

Using [rocket](https://crates.io/crates/rocket) and [rocket-prometheus](https://crates.io/crates/rocket-prometheus)

Requires nightly:

```console
Error: Rocket (codegen) requires a 'dev' or 'nightly' version of rustc.
Installed version: 1.46.0 (2020-08-24)
Minimum required:  1.33.0-nightly (2019-01-13)
```

Install nightly:

```bash
rustup toolchain install nightly
```

Use nightly for this (!) project only:

```bash
cd ${HOME}/Projects/do-apps-rust
rustup override set nightly
cargo run
```

Or:

```bash
docker build --tag=do-apps-rust --file=./Dockerfile .
```

Then:

```bash
CONT=7777
docker run \
--interactive --tty \
--env=PORT=${CONT} \
--publish=8888:${CONT} \
do-apps-rust
```

> **NOTE** The server binds to the value of `${PORT}`

Then e.g.:

+ `http://localhost/hello/freddie/2`
+ `http://localhost/env`
+ `http://localhost/headers`
+ `http://localhost/healthz`
+ `http://localhost/metrics`


