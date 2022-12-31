# `apollo`

Apollo 绘图引擎的稳定公共 Rust 门面。A1 起 re-export `ColumnTable`、`PlotSpec` 与 `validate_plot`。

可选 feature：

- `wgpu` — 链接 `apollo-backend-wgpu`

```sh
cargo test -p apollo
cargo test -p apollo --features wgpu
```
