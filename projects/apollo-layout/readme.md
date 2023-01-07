# `apollo-layout`

2D 面板布局与图/树布局：

- 分面：`layout_facet_panels` / `PanelRect`
- 图：`CircularLayout` / `GridLayout` / `LayeredLayout` / `ForceLayout`
- 树：`TidyTreeLayout` / `RadialTreeLayout`

输出 `LayoutResult`（位置 + 边路由），不生成 Scene 节点。

```sh
cargo test -p apollo-layout
```
