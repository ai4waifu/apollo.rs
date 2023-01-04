//! Apollo 渲染器合同 — CPU、SVG、GPU 后端共用。
//!
//! 只消费 Scene IR。不得重新推断 stat、scale 或 layout。

#![deny(missing_docs)]

mod cpu;
mod prepare;
mod report;
mod svg;
mod target;
mod walk;

pub use cpu::CpuRasterRenderer;
pub use prepare::PreparedScene;
pub use report::{Capability, FrameReport, RendererCapabilities};
pub use svg::SvgRenderer;
pub use target::{RenderTarget, RgbaImage};

use apollo_scene::Scene;
use apollo_types::Result;

/// 渲染器合同：只消费已验证的 Scene IR。
pub trait Renderer {
    /// 能力探测。
    fn capabilities(&self) -> RendererCapabilities;

    /// 准备场景（A2：克隆持有，后续可做资源上传）。
    fn prepare(&mut self, scene: &Scene) -> Result<PreparedScene>;

    /// 渲染到目标。
    fn render(&mut self, scene: &PreparedScene, target: &mut RenderTarget) -> Result<FrameReport>;
}

/// 便捷：Scene → RGBA8 位图。
pub fn render_rgba8(scene: &Scene) -> Result<RgbaImage> {
    let mut renderer = CpuRasterRenderer::new();
    let prepared = renderer.prepare(scene)?;
    let mut target = RenderTarget::Rgba8(RgbaImage::from_viewport(scene.viewport));
    renderer.render(&prepared, &mut target)?;
    match target {
        RenderTarget::Rgba8(image) => Ok(image),
        RenderTarget::Svg(_) => unreachable!("cpu renderer only writes rgba8"),
    }
}

/// 便捷：Scene → SVG 文档字符串。
pub fn render_svg(scene: &Scene) -> Result<String> {
    let mut renderer = SvgRenderer::new();
    let prepared = renderer.prepare(scene)?;
    let mut target = RenderTarget::Svg(String::new());
    renderer.render(&prepared, &mut target)?;
    match target {
        RenderTarget::Svg(document) => Ok(document),
        RenderTarget::Rgba8(_) => unreachable!("svg renderer only writes svg"),
    }
}
