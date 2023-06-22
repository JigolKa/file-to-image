use image::{ImageBuffer, Rgb, RgbImage};
use std::convert::TryInto;
use std::env;
use std::fs;
use std::process;

fn get_rgb(value: u16) -> [u8; 3] {
    let red = ((value >> 10) & 0xff) as u8;
    let green = ((value >> 6) & 0xff) as u8;
    let blue = ((value >> 2) & 0xff) as u8;

    [red, green, blue]
}

fn get_colors(contents: &String) -> Vec<[u8; 3]> {
    let mut colors: Vec<[u8; 3]> = vec![];

    for c in contents.chars() {
        let base: u16 = 256;
        colors.push(get_rgb((c as u16) * base));
    }

    colors
}

fn read_file(path: &String) -> String {
    let contents = fs::read_to_string(path).expect("Should have been able to read the file");

    contents
}

fn factors(n: &mut i32) -> Vec<i32> {
    let mut out = vec![];

    for i in 2..(*n + 1) {
        while *n % i == 0 {
            out.push(i);

            *n /= i;
        }

        if *n == 1 {
            break;
        }
    }

    out
}

fn get_dimensions(vec_len: &mut i32) -> [i32; 2] {
    let factors = factors(vec_len);
    let len = factors.len();

    let mut height = 1;
    let _ = &factors[..len - 1].into_iter().for_each(|x| height *= *x);

    [*factors.last().expect("msg"), height]
    // TODO: Create proper square
}

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: ./program {{path}} {{out=output.png}}");
        process::exit(1);
    }

    let file_path = &args[1];
    let contents = read_file(&file_path);
    let colors = get_colors(&contents);
    let len = colors.len();
    let colors_iter = &mut colors.into_iter();

    let mut size = i32::try_from(len).ok().unwrap();
    let dimensions = get_dimensions(&mut size);

    let mut image: RgbImage = ImageBuffer::new(dimensions[0] as u32, dimensions[1] as u32);

    let mut y = 0;
    for (i, c) in colors_iter.enumerate() {
        let index: i32 = i.try_into().unwrap();
        let x = index % dimensions[0];

        if x == 0 && i != 0 {
            y += 1;
        }

        image.put_pixel(x as u32, y, Rgb(c));
    }

    let save_path = if args.len() > 2 && !args[2].is_empty() {
        &args[2]
    } else {
        "output.png"
    };
    image.save(save_path).unwrap();

    println!("Image saved at {}", save_path);
}
