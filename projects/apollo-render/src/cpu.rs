//! CPU 栅格后端 — 确定性 reference。

use apollo_scene::{AxisNode, Point2, PolylineNode, Scene};
use apollo_types::{Diagnostic, DiagnosticCode, Result, Rgba};

use crate::{
    Renderer,
    prepare::PreparedScene,
    report::{Capability, FrameReport, RendererCapabilities},
    target::RenderTarget,
    walk::{Drawable, walk_drawables},
};

/// CPU 栅格渲染器。
#[derive(Debug, Default, Clone, Copy)]
pub struct CpuRasterRenderer;

impl CpuRasterRenderer {
    /// 构造。
    pub const fn new() -> Self {
        Self
    }
}

impl Renderer for CpuRasterRenderer {
    fn capabilities(&self) -> RendererCapabilities {
        RendererCapabilities { raster_2d: Capability::Available, svg: Capability::Unsupported, gpu: Capability::Unsupported }
    }

    fn prepare(&mut self, scene: &Scene) -> Result<PreparedScene> {
        Ok(PreparedScene::from_scene(scene))
    }

    fn render(&mut self, prepared: &PreparedScene, target: &mut RenderTarget) -> Result<FrameReport> {
        let RenderTarget::Rgba8(image) = target
        else {
            return Err(Diagnostic::error(DiagnosticCode::UnsupportedTarget, "CpuRasterRenderer 需要 Rgba8 目标"));
        };

        let scene = &prepared.scene;
        *image = crate::target::RgbaImage::from_viewport(scene.viewport);
        let height = image.height as f64;
        let mut primitive_count = 0_u32;

        walk_drawables(scene, |drawable| {
            match drawable {
                Drawable::Polyline(polyline) => {
                    primitive_count += draw_polyline(image, height, polyline);
                }
                Drawable::Axis(axis) => {
                    primitive_count += draw_axis(image, height, axis);
                }
            }
            Ok(())
        })?;

        Ok(FrameReport { primitive_count })
    }
}

fn scene_to_pixel(point: Point2, height: f64) -> (i32, i32) {
    // 场景 y 向上，位图 y 向下。
    ((point.x).round() as i32, (height - point.y).round() as i32)
}

fn draw_polyline(image: &mut crate::target::RgbaImage, height: f64, polyline: &PolylineNode) -> u32 {
    if polyline.points.len() < 2 {
        return 0;
    }
    let radius = ((polyline.linewidth.max(1.0) - 1.0) * 0.5).ceil() as i32;
    let mut count = 0_u32;
    for window in polyline.points.windows(2) {
        let (x0, y0) = scene_to_pixel(window[0], height);
        let (x1, y1) = scene_to_pixel(window[1], height);
        count += draw_thick_line(image, x0, y0, x1, y1, radius, polyline.stroke);
    }
    count
}

fn draw_axis(image: &mut crate::target::RgbaImage, height: f64, axis: &AxisNode) -> u32 {
    let mut count = 0_u32;
    let origin = axis.origin;
    let end = if axis.horizontal {
        Point2::new(origin.x + axis.length, origin.y)
    }
    else {
        Point2::new(origin.x, origin.y + axis.length)
    };
    let (x0, y0) = scene_to_pixel(origin, height);
    let (x1, y1) = scene_to_pixel(end, height);
    count += draw_thick_line(image, x0, y0, x1, y1, 0, axis.stroke);

    let ticks = axis.tick_count.max(2);
    for i in 0..ticks {
        let t = f64::from(i) / f64::from(ticks - 1);
        let (tick_start, tick_end) = if axis.horizontal {
            let x = origin.x + axis.length * t;
            (Point2::new(x, origin.y), Point2::new(x, origin.y - 6.0))
        }
        else {
            let y = origin.y + axis.length * t;
            (Point2::new(origin.x, y), Point2::new(origin.x - 6.0, y))
        };
        let (sx, sy) = scene_to_pixel(tick_start, height);
        let (ex, ey) = scene_to_pixel(tick_end, height);
        count += draw_thick_line(image, sx, sy, ex, ey, 0, axis.stroke);
    }
    count
}

fn draw_thick_line(image: &mut crate::target::RgbaImage, x0: i32, y0: i32, x1: i32, y1: i32, radius: i32, color: Rgba) -> u32 {
    let mut count = 0_u32;
    for point in bresenham(x0, y0, x1, y1) {
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                image.set(point.0 + dx, point.1 + dy, color);
            }
        }
        count += 1;
    }
    count
}

fn bresenham(mut x0: i32, mut y0: i32, x1: i32, y1: i32) -> Vec<(i32, i32)> {
    let mut points = Vec::new();
    let dx = (x1 - x0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let dy = -(y1 - y0).abs();
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx + dy;
    loop {
        points.push((x0, y0));
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            err += dx;
            y0 += sy;
        }
    }
    points
}
