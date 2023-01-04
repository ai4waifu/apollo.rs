# `apollo-render`

CPU、SVG 与 GPU 后端共用的渲染器合同。A2 提供：

- `Renderer` trait
- `CpuRasterRenderer`（确定性 RGBA8 reference）
- `SvgRenderer`（静态矢量导出）

后端只消费 Scene IR，不接收 `PlotSpec`。

```sh
cargo test -p apollo-render
```
