//! 折线/轴线 → 线段顶点。

use apollo_render::{Drawable, walk_drawables};
use apollo_scene::{AxisNode, Point2, PolylineNode, Scene};
use apollo_types::{Result, Rgba};

/// 线段列表顶点（场景坐标，y 向上）。
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LineVertex {
    /// xy。
    pub position: [f32; 2],
    /// rgba。
    pub color: [f32; 4],
}

impl LineVertex {
    fn new(point: Point2, color: Rgba) -> Self {
        Self { position: [point.x as f32, point.y as f32], color: [color.r, color.g, color.b, color.a] }
    }
}

/// 收集 Scene 中全部线段（折线邻接边 + 坐标轴与刻度）。
pub fn collect_line_vertices(scene: &Scene) -> Result<Vec<LineVertex>> {
    let mut vertices = Vec::new();
    walk_drawables(scene, |drawable| {
        match drawable {
            Drawable::Polyline(polyline) => push_polyline(&mut vertices, polyline),
            Drawable::Axis(axis) => push_axis(&mut vertices, axis),
        }
        Ok(())
    })?;
    Ok(vertices)
}

fn push_segment(out: &mut Vec<LineVertex>, a: Point2, b: Point2, color: Rgba) {
    out.push(LineVertex::new(a, color));
    out.push(LineVertex::new(b, color));
}

fn push_polyline(out: &mut Vec<LineVertex>, polyline: &PolylineNode) {
    for window in polyline.points.windows(2) {
        push_segment(out, window[0], window[1], polyline.stroke);
    }
}

fn push_axis(out: &mut Vec<LineVertex>, axis: &AxisNode) {
    let origin = axis.origin;
    let end = if axis.horizontal {
        Point2::new(origin.x + axis.length, origin.y)
    }
    else {
        Point2::new(origin.x, origin.y + axis.length)
    };
    push_segment(out, origin, end, axis.stroke);

    let ticks = axis.tick_count.max(2);
    for i in 0..ticks {
        let t = f64::from(i) / f64::from(ticks - 1);
        let (start, tip) = if axis.horizontal {
            let x = origin.x + axis.length * t;
            (Point2::new(x, origin.y), Point2::new(x, origin.y - 6.0))
        }
        else {
            let y = origin.y + axis.length * t;
            (Point2::new(origin.x, y), Point2::new(origin.x - 6.0, y))
        };
        push_segment(out, start, tip, axis.stroke);
    }
}
