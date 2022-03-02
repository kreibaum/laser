use std::f64::consts::PI;

use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use svg::Document;

// Math Constants
const SQRT_3: f64 = 1.73205080757;
const TAN_30: f64 = 0.57735026919;

// Size Constants
const HEX_WIDTH: f64 = 200.0;
const HEX_RADIUS: f64 = (HEX_WIDTH / 2.0) * 2.0 / SQRT_3;

const INNER_SPACE: f64 = 20.0;
const CORNER_INSET: f64 = INNER_SPACE * TAN_30;
const CORNER_INSET_REL: f64 = CORNER_INSET / HEX_RADIUS;

// Design Constants
const TOP_WIDTH: i64 = 5;
const HEIGHT: i64 = 3;

// Returns data for a hexagon centered on (0, 0) with the given radius.
// Flat sides are parallel to the y-axis.
fn hex_data() -> Data {
    let hex_points = hex_points_array();

    let mut data = Data::new();
    data = data.move_to(lerp(hex_points[0], hex_points[1], CORNER_INSET_REL));
    for i in 1..7 {
        // Slightly shorter outer line where we substract relative corner inset.
        let c1 = lerp(hex_points[i - 1], hex_points[i], 1.0 - CORNER_INSET_REL);
        data = data.line_to(c1);
        // And now we already step into the next side.
        // But instead of line_to, we'll sweep with an arc.
        let (c2x, c2y) = lerp(hex_points[i], hex_points[i + 1], CORNER_INSET_REL);
        // data = data.line_to(c2);
        data = data.elliptical_arc_to((INNER_SPACE, INNER_SPACE, 0.0, 0.0, 1.0, c2x, c2y));
    }
    data.close()
}

#[allow(clippy::needless_range_loop)] // It is easier to read this way.
fn hex_points_array() -> [(f64, f64); 12] {
    let mut hex_points: [(f64, f64); 12] = [(0.0, 0.0); 12];
    for i in 0..12 {
        let angle = i as f64 * PI / 3.0 + PI / 6.0;
        hex_points[i] = (HEX_RADIUS * angle.cos(), HEX_RADIUS * angle.sin());
    }
    hex_points
}

fn lerp(a: (f64, f64), b: (f64, f64), t: f64) -> (f64, f64) {
    (a.0 + (b.0 - a.0) * t, a.1 + (b.1 - a.1) * t)
}

fn outline(data: Data) -> Path {
    Path::new()
        .set("d", data)
        .set("stroke", "black")
        .set("stroke-width", "3")
        .set("fill", "none")
}

trait Transformable: Sized {
    /// Arbitrary translation.
    fn translate(self, x: f64, y: f64) -> Self;
    /// Translation on a hex grid.
    fn hex_translate(self, x: i64, y: i64) -> Self {
        self.translate(
            (x as f64 + y as f64 / 2.0) * (HEX_WIDTH + INNER_SPACE),
            y as f64 * (HEX_WIDTH + INNER_SPACE) / 2.0 * SQRT_3,
        )
    }
}

impl Transformable for Group {
    fn translate(self, x: f64, y: f64) -> Self {
        self.set("transform", format!("translate({}, {})", x, y))
    }
}

fn main() {
    // Remember that the coordinate system is flipped for svg.
    // We don't want to deal with that, so we'll introduce a mirrored group.
    let w = 1400;
    let h = 800;

    let mut document = Document::new().set("viewBox", (0, 0, w, h)); // x, y, width, height
    let mut mirror_group = Group::new().set("transform", format!("matrix(1,0,0,-1,0,{h})"));

    for j in 0..HEIGHT {
        for i in 0..(TOP_WIDTH - j) {
            let path = outline(hex_data());
            let g = Group::new().add(path).hex_translate(i + j, -j);
            let g = Group::new().add(g).translate(200.0, 600.0);
            mirror_group = mirror_group.add(g);
        }
    }

    document = document.add(mirror_group);
    svg::save("output/image.svg", &document).unwrap();

    println!("Hello, world!");
}
