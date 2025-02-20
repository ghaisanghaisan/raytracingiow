use std::io::{BufWriter, Write};

use crate::{interval::Interval, vec3::Color};

pub fn write_color<W: Write>(w: &mut BufWriter<W>, color: &Color) {
    let color_interval = Interval::new(0.0, 0.999);
    let rbyte = (color_interval.clamp(color.x()) * 256.0) as i32;
    let gbyte = (color_interval.clamp(color.y()) * 256.0) as i32;
    let bbyte = (color_interval.clamp(color.z()) * 256.0) as i32;

    writeln!(w, "{} {} {}\n", rbyte, gbyte, bbyte).unwrap();
}
