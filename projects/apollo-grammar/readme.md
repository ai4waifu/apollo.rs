# `apollo-grammar`

声明式 Grammar of Graphics。A1/A2 可构造、校验并编译：

```text
ColumnTable → PlotSpec { x, y, geom_line } → Scene { axis, polyline }
```

拥有 `PlotSpec` 与编译入口。不拥有渲染后端。

```sh
cargo test -p apollo-grammar
```
