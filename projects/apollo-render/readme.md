# `apollo-render`

CPU 栅格、SVG 与 GPU 后端共用的 renderer trait 与合同。

后端只消费 Scene IR，不得重新推断 grammar、scale 或 layout。

```sh
cargo test -p apollo-render
```
