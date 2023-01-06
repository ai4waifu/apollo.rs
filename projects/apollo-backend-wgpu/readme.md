# `apollo-backend-wgpu`

可选 WGPU / WebGPU 后端。A4 首切片：离屏渲染折线与坐标轴到 RGBA8，便于与 CPU reference 对照。

默认不进入 `apollo` 核心依赖；经 `apollo` 的 `wgpu` feature 启用。

```sh
cargo test -p apollo-backend-wgpu
cargo test -p apollo --features wgpu
```
