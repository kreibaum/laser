use std::f64::consts::PI;

use svg::node::element::path::Data;
use svg::node::element::{Group, Path};
use svg::Document;

const SQRT_3: f64 = 1.73205080757;

const HEX_WIDTH: f64 = 200.0;
const HEX_RADIUS: f64 = (HEX_WIDTH / 2.0) * 2.0 / SQRT_3;

const INNER_SPACE: f64 = 20.0;

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
    // let path1 = outline(hex_data(40.0, 5.0));

    let mut document = Document::new().set("viewBox", (-400, -400, 800, 800)); // x, y, w, h

    for i in -1..=1 {
        let path2 = outline(hex_data(HEX_RADIUS));
        let g = Group::new()
            .add(path2)
            //.translate((i as f64) * (HEX_WIDTH + INNER_SPACE), 0.0);
            .hex_translate(0, i);
        document = document.add(g);

        let path2 = outline(hex_data(HEX_RADIUS));
        let g = Group::new()
            .add(path2)
            //.translate((i as f64) * (HEX_WIDTH + INNER_SPACE), 0.0);
            .hex_translate(i, 0);
        document = document.add(g);
    }

    svg::save("output/image.svg", &document).unwrap();

    println!("Hello, world!");
}
