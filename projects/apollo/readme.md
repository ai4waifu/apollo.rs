# `apollo`

Apollo 绘图引擎的稳定公共 Rust 门面。负责 re-export 各层合同，并在垂直切片落地后托管 builder。

可选 feature：

- `wgpu` — 链接 `apollo-backend-wgpu`

```sh
cargo test -p apollo
cargo test -p apollo --features wgpu
```
