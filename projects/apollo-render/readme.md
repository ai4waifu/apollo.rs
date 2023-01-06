# `apollo-render`

渲染器合同与 CPU 确定性 reference：

- `Renderer` trait
- `PreparedScene` / `RenderTarget` / `RgbaImage`
- `CpuRasterRenderer` / `render_rgba8`
- `walk_drawables`（供各 backend 复用）

SVG 与 WGPU 分别见 `apollo-backend-svg`、`apollo-backend-wgpu`。

```sh
cargo test -p apollo-render
```
