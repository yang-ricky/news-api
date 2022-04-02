## news-api

## How to build?
  *  cargo build

## How to run ?
  * RUST_LOG=info cargo run --bin news-api

## How to deploy?
  * kubectl apply -f deployment.yaml


## 概念
  * 一个crate 就是一个模块

##  TODO
  * 集成JSON库
  * 集成news api
  * 属性验证

## Resource
  * [模块列表](https://www.processon.com/view/link/62482e7f5653bb072bd7979e)

## Other
Update mirror in ~/.cargo/config
```
   [source.crates-io]
   registry = "https://github.com/rust-lang/crates.io-index"
  
   replace-with = 'tuna'
   [source.tuna]
   registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"
  
   [net]
   git-fetch-with-cli = true
```
