use std::f64::consts::PI;

use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use svg::Document;

// Math Constants
const SQRT_3: f64 = 1.73205080757;

// Size Constants
const HEX_WIDTH: f64 = 200.0;
const HEX_RADIUS: f64 = (HEX_WIDTH / 2.0) * 2.0 / SQRT_3;

const INNER_SPACE: f64 = 20.0;

// Design Constants
const TOP_WIDTH: i64 = 5;
const HEIGHT: i64 = 3;

// Returns data for a hexagon centered on (0, 0) with the given radius.
// Flat sides are parallel to the y-axis.
fn hex_data(radius: f64) -> Data {
    let mut data = Data::new();
    let mut angle: f64 = PI / 6.0;
    data = data.move_to((radius * angle.cos(), radius * angle.sin()));
    for _ in 1..7 {
        data = data.line_to((radius * angle.cos(), radius * angle.sin()));
        angle += PI / 3.0;
    }
    data.close()
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
            let path = outline(hex_data(HEX_RADIUS));
            let g = Group::new().add(path).hex_translate(i + j, -j);
            let g = Group::new().add(g).translate(200.0, 600.0);
            mirror_group = mirror_group.add(g);
        }
    }

    document = document.add(mirror_group);
    svg::save("output/image.svg", &document).unwrap();

    println!("Hello, world!");
}
