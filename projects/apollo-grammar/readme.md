# `apollo-grammar`

声明式 Grammar of Graphics。A1/A2 可构造、校验并编译：

```text
ColumnTable | GridData | GraphData | TreeData → PlotSpec → Scene
```

编译链路：`validate → scale/layout → scene`（图/树经 `apollo-layout`）。

拥有 `PlotSpec` 与编译入口。不拥有渲染后端。

```sh
cargo test -p apollo-grammar
```
