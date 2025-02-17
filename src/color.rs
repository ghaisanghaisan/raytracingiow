use std::io::{BufWriter, Write};

use crate::vec3::Color;

pub fn write_color<W: Write>(w: &mut BufWriter<W>, color: &Color) {
    let rbyte = (color.x() * 255.999) as i32;
    let gbyte = (color.y() * 255.999) as i32;
    let bbyte = (color.z() * 255.999) as i32;

    writeln!(w, "{} {} {}\n", rbyte, gbyte, bbyte).unwrap();
}
