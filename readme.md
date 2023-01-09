# Apollo

Apollo 是基于 Grammar of Graphics 的纯 Rust 绘图引擎。它将声明式图形规格编译为后端无关的 Scene IR，再通过 CPU、SVG 或可选 GPU 后端渲染。

上层宿主可单向依赖 Apollo，把已准备好的数据 lowering 为 Apollo 数据 / `PlotSpec`。本引擎不求值数学表达式，也不依赖任何 CAS。

## Crate 分层

```text
apollo-types → apollo-data → apollo-grammar
                 ↘
                  apollo-scene → apollo-layout
                              → apollo-render → apollo-backend-svg（默认）
                                              → apollo-backend-wgpu（可选）
                                              → apollo（门面）
```

| Crate | 职责 |
|-------|------|
| [`apollo-types`](projects/apollo-types/readme.md) | ID、颜色、单位、范围、诊断 |
| [`apollo-data`](projects/apollo-data/readme.md) | 列式表、视图、缺失值 |
| [`apollo-grammar`](projects/apollo-grammar/readme.md) | PlotSpec / ggplot 风格图形语法 |
| [`apollo-scene`](projects/apollo-scene/readme.md) | 后端无关 Scene IR |
| [`apollo-layout`](projects/apollo-layout/readme.md) | 2D/3D、图、树与标签布局 |
| [`apollo-render`](projects/apollo-render/readme.md) | 渲染器合同与 CPU reference |
| [`apollo-backend-svg`](projects/apollo-backend-svg/readme.md) | SVG 矢量后端 |
| [`apollo-backend-wgpu`](projects/apollo-backend-wgpu/readme.md) | 可选 WGPU / WebGPU 后端 |
| [`apollo`](projects/apollo/readme.md) | 稳定公共 Rust 门面 |

## 开发

通过 `rust-toolchain.toml` 使用 nightly 工具链。

```sh
cargo test --workspace
cargo test -p apollo --features wgpu
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
```

默认构建链接 SVG，不链接 GPU SDK。需要 GPU 时启用 `wgpu` feature。

## 许可证

MPL-2.0，见 [`License.md`](License.md)。
