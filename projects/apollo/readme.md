# `apollo`

Apollo 绘图引擎的稳定公共 Rust 门面。A2 起 re-export `PlotSpec`、`compile_plot`、Scene IR，以及 CPU / SVG 渲染入口。

可选 feature：

- `wgpu` — 链接 `apollo-backend-wgpu`

```sh
cargo test -p apollo
cargo test -p apollo --features wgpu
```
