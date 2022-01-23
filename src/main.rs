use std::fs::File;

use tiff::encoder::{colortype, TiffEncoder};

fn mandelbrot(r: f32, i: f32) -> u16 {
    let mut r0: f32 = 0.0;
    let mut i0: f32 = 0.0;
    let mut c: u16 = 0;
    while (r0 * r0 + i0 * i0) < 4.0 {
        if c == 0xffff {
            return 0;
        }
        c += 1;
        let t: f32 = r0 * r0 - i0 * i0 + r;
        i0 = 2.0 * r0 * i0 + i;
        r0 = t;
    }
    return c;
}

fn main() {
    let size: usize = 3000;
    let step: f32 = 3.0 / size as f32;
    let mut ms: Vec<u16> = Vec::with_capacity(size * size);
    for _i in 0..size {
        let i: f32 = -1.5 + step * _i as f32;
        for _r in 0..size {
            let r: f32 = -2.0 + step * _r as f32;
            ms.push(mandelbrot(r, i));
        }
        println!("{} of {}", _i, size);
    }
    let file = File::create("mandelbrot.tiff").unwrap();
    let mut tiff = TiffEncoder::new(&file).unwrap();
    tiff.write_image::<colortype::Gray16>(size as u32, size as u32, &ms)
        .unwrap();
}
