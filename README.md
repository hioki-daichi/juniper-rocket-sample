# juniper-rocket-sample

juniper-rocket-sample is a sample app using `juniper` of GraphQL library and `rocket` of web framework library.

![](https://raw.githubusercontent.com/wiki/hioki-daichi/juniper-rocket-sample/demo.gif)

**NOTE:** This repository is API only. The frontend is in a separate repository: [https://github.com/hioki-daichi/juniper-rocket-sample-frontend](https://github.com/hioki-daichi/juniper-rocket-sample-frontend)

## Requirement

- [docker-compose](https://github.com/docker/compose)
- [direnv](https://github.com/direnv/direnv)
- [forego](https://github.com/ddollar/forego)
- [cargo-make](https://github.com/sagiegurari/cargo-make)

## Usage

```sh
$ cargo install --force cargo-make
$ docker-compose up
$ cargo make setup
$ forego start
```
