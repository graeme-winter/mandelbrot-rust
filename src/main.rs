use std::fs::File;

use tiff::encoder::{colortype, TiffEncoder};

fn mandelbrot(r: f32, i: f32) -> u16 {
    let mut r0: f32 = 0.0;
    let mut i0: f32 = 0.0;
    let mut c: u16 = 0;
    while (r0 * r0 + i0 * i0) < 4.0 {
        if c == 32767 {
            break;
        }
        c += 1;
        let t: f32 = r0 * r0 - i0 * i0 + r;
        i0 = 2.0 * r0 * i0 + i;
        r0 = t;
    }
    return c;
}

fn main() {
    // scan along i = 1.0 from -2 to +2, steps of 0.01
    let mut ms: [[u16; 400]; 400] = [[0; 400]; 400];
    for _i in 0..400 {
        let i: f32 = -2.0 + 0.01 * _i as f32;
        for _r in 0..400 {
            let r: f32 = -2.0 + 0.01 * _r as f32;
            ms[_i][_r] = mandelbrot(r, i)
        }
    }
    let data: Vec<u16> = ms.iter().flatten().cloned().collect();
    let file = File::create("mandelbrot.tiff").unwrap();
    let mut tiff = TiffEncoder::new(&file).unwrap();
    tiff.write_image::<colortype::Gray16>(400, 400, &data)
        .unwrap();
}
