use std::{
    fs::File,
    io::{BufWriter, Write},
};

fn main() {
    let size = 500;
    let center = size / 2;
    let file = File::create("output.ppm").unwrap();
    let mut writer = BufWriter::new(file);

    writeln!(writer, "P3\n{} {}\n255\n", size, size).unwrap();
    for y in 0..size {
        println!("Generated Rows: {} / {}", y, size);
        for x in 0..size {
            let u = x - center;
            let v = center - y;

            if u * u + v * v <= 10000 {
                let intensity = 255 * x / size;
                writeln!(writer, "255 {} 255\n", intensity).unwrap();
            } else {
                writeln!(writer, "0 0 0\n").unwrap();
            }
        }
    }

    writer.flush().unwrap();

    println!("Done!");
}
