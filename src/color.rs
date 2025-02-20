use std::io::{BufWriter, Write};

use crate::{interval::Interval, vec3::Color};

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color<W: Write>(w: &mut BufWriter<W>, color: &Color) {
    let color_interval = Interval::new(0.0, 0.999);
    let r = linear_to_gamma(color.x());
    let g = linear_to_gamma(color.y());
    let b = linear_to_gamma(color.z());
    let rbyte = (color_interval.clamp(r) * 256.0) as i32;
    let gbyte = (color_interval.clamp(g) * 256.0) as i32;
    let bbyte = (color_interval.clamp(b) * 256.0) as i32;

    writeln!(w, "{} {} {}\n", rbyte, gbyte, bbyte).unwrap();
}
