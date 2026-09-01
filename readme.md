# Apollo 绘图引擎

Apollo 是纯 Rust 的声明式绘图引擎，采用 Grammar of Graphics 思路，将数据与 `PlotSpec` 编译为后端无关的 Scene IR，再交给
CPU、SVG 或可选的 WGPU/WebGPU 后端绘制。

Apollo 只负责把已经准备好的数据画出来，不负责数学表达式求值、符号化简、函数采样、数据集处理或业务语义，也不依赖任何计算内核。调用方完成求值与采样后，再把列式数据、网格、图或树交给 Apollo。

## 维护者入口

实现入口是 `projects/apollo/src/lib.rs`，它是稳定公共 facade。各模块 README 与源码测试分别说明类型、数据、语法、场景、布局和渲染职责。新增能力先确定所属
crate，再补公共合同和测试，不能为了方便把逻辑堆进 facade。

## 数据流

```text
数据 / GraphData / TreeData / GridData
    → PlotSpec
    → validate → stat → scale → layout
    → Scene IR
    → CPU / SVG / WGPU renderer
```

`PlotSpec` 描述数据如何映射到视觉通道，以及使用哪些统计变换、几何对象、尺度、坐标系和主题。Scene IR
描述已布局的绘制节点，是渲染器唯一应接收的输入。渲染阶段不得重新推断统计、尺度或布局。

## Crate 分层

```text
apollo-types → apollo-data → apollo-grammar → apollo-scene
                                      │             │
                                      └──────────→ apollo-layout
                                                      │
                                                apollo-render
                                                  │       │
                                   apollo-backend-svg   apollo-backend-wgpu
                                                  \       /
                                                     apollo
```

| Crate                 | 职责                                                                  | 不负责                          |
|-----------------------|-----------------------------------------------------------------------|---------------------------------|
| `apollo-types`        | ID、颜色、范围、单位、诊断、版本合同                                  | 数据处理、布局、GPU 生命周期    |
| `apollo-data`         | 列式表、缺失值、视图、流式批次、图、树、网格                          | 图形语法、数学求值              |
| `apollo-grammar`      | mapping、stat、geom、scale、coordinate、facet、theme、`PlotSpec` 编译 | 具体后端绘制                    |
| `apollo-scene`        | 后端无关 Scene IR、节点 arena、资源、相机、拾取信息                   | ggplot 语义、设备 API           |
| `apollo-layout`       | 面板、2D/3D、图、树、标签和约束布局                                   | GPU 资源管理                    |
| `apollo-render`       | renderer trait、能力报告、CPU reference renderer                      | 方言、CAS IR、具体 GPU/SVG 实现 |
| `apollo-backend-svg`  | SVG 矢量输出                                                          | 图形语法、布局、GPU 生命周期    |
| `apollo-backend-wgpu` | WGPU/WebGPU 资源、shader、pipeline                                    | 图形语法、布局语义              |
| `apollo`              | 稳定 facade、feature 汇总、便捷导出                                   | 重复实现各层逻辑、调用方适配、计算内核 |

第一阶段允许物理目录暂时合并，但职责和依赖方向不能合并。不要为每个 geom、布局算法或设备建立独立 crate。

## 最小使用形状

```rust
use apollo::{aes, compile_plot, geom_line, ColumnTable, FloatColumn};

let data = ColumnTable::new()
.with_column("x", FloatColumn::from(vec![0.0, 1.0, 2.0]))
.with_column("y", FloatColumn::from(vec![0.0, 1.0, 4.0]));

let spec = apollo::PlotSpec::builder(data)
.mapping(aes().x("x").y("y"))
.layer(geom_line())
.build() ?;
let scene = compile_plot( & spec) ?;
```

示例只展示调用方向，具体 builder 名称以当前公共 API 和测试为准。新增示例时，优先补充仓库测试中的合同测试，不在 README
中承诺尚未实现的 API。

## 后端与 feature

默认 feature 是 `svg`，核心构建不强制链接系统 GPU SDK。需要 WGPU 时启用 `wgpu` feature：

```sh
cargo test --workspace
cargo test -p apollo --features wgpu
```

后端能力用结构化 capability 报告表达。没有 GPU 时应降级到 CPU 或 SVG。`apollo-scene` 和 `apollo-render` 不得泄漏 `wgpu`
类型。

## 当前实现状态

已具备 2D 基础图元与 golden 测试、continuous/log scale、笛卡尔坐标、facet、主题、3D 相机与网格、曲面与点云、CPU 拾取、图与树布局，以及
CPU/SVG/WGPU 基础 renderer。

仍在路线中的工作包括完整 CPU/GPU primitive parity、WASM WebGPU smoke、流式增量资源、动画、PNG/PDF 导出和调用方采样适配。规划中的能力不能写成已完成。

## 维护规则

1. 先验证 `PlotSpec`，再生成 Scene IR，最后渲染。
2. renderer 只消费 Scene IR，不重新执行 stat、scale 或 layout。
3. 采样、CAS 求值、函数语义和宿主生命周期放在 Apollo 之外。
4. Scene IR 不得包含 CAS `TermId`、方言 AST、Session 或业务对象。
5. CPU 保持 correctness reference，GPU 只作为可选加速路径。
6. 诊断使用结构化 `code + args + details + span`，不要把自然语言 message 当作跨后端合同。
7. 新公共能力先写清公共合同，再实现并补测试。

## 验证命令

```sh
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo test -p apollo-backend-wgpu --features wgpu
```

WGPU/WASM 测试依赖运行环境。没有 GPU 时应验证 capability/fallback 路径，不能用跳过测试冒充后端完成。

## 许可证

Apache-2.0，详见仓库根目录 `license.md`。
