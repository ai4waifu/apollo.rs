# `apollo`

Apollo 绘图引擎的稳定公共 Rust 门面。A1/A2 起 re-export `ColumnTable`、`PlotSpec`、`compile_plot` 与 Scene IR。

可选 feature：

- `wgpu` — 链接 `apollo-backend-wgpu`

```sh
cargo test -p apollo
cargo test -p apollo --features wgpu
```
