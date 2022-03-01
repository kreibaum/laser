use std::f64::consts::PI;

use svg::node::element::path::Data;
use svg::node::element::Path;
use svg::Document;

const HEX_RADIUS: f64 = 120.0;

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

fn main() {
    // let path1 = outline(hex_data(40.0, 5.0));

    let path2 = outline(hex_data(HEX_RADIUS));

    let document = Document::new()
        .set("viewBox", (-400, -400, 800, 800)) // x, y, w, h
        // .add(path1)
        .add(path2);

    svg::save("output/image.svg", &document).unwrap();

    println!("Hello, world!");
}
