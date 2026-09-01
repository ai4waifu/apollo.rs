# `apollo-render`

渲染器合同与 CPU 确定性 reference：

- `Renderer` trait
- `PreparedScene` / `RenderTarget` / `RgbaImage`
- `CpuRasterRenderer` / `render_rgba8`
- `walk_drawables`（供各 backend 复用）

本 crate 只定义渲染器边界和 CPU 参考实现。SVG 与 WGPU 是独立后端，分别位于 `apollo-backend-svg` 和 `apollo-backend-wgpu`
，不会把后端类型泄漏到 Scene IR。

```sh
cargo test -p apollo-render
```
