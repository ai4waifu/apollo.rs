# `apollo-grammar`

声明式 Grammar of Graphics。A1 可构造并校验：

```text
ColumnTable → PlotSpec { x, y, geom_line } + continuous scale + cartesian2d
```

拥有 `PlotSpec`。不拥有 Scene IR 或渲染后端。

```sh
cargo test -p apollo-grammar
```
