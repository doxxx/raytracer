extern crate image;
extern crate clap;
extern crate wavefront_obj;
extern crate pbr;
extern crate num_cpus;
extern crate time;

mod color;
mod direction;
mod lights;
mod matrix;
mod object;
mod point;
mod scene;
mod shaders;
mod shapes;
mod system;
mod texture;
mod vector;

use std::fs::File;

use clap::{App, Arg};

use color::Color;
use system::Options;

fn number_validator(s: String) -> Result<(), String> {
    if s.parse::<usize>().is_ok() { return Ok(()); }
    Err(String::from("The value must be a number."))
}

fn main() {
    let default_cpus = format!("{}", num_cpus::get() - 1);
    let app = App::new("raytracer")
        .version("0.1.0")
        .author("Gordon Tyler <gordon@doxxx.net>")
        .about("Simple ray tracer")
        .arg(
            Arg::with_name("width")
                .short("w")
                .value_name("WIDTH")
                .help("Image width")
                .takes_value(true)
                .validator(number_validator)
                .default_value("1024"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .value_name("HEIGHT")
                .help("Image height")
                .takes_value(true)
                .validator(number_validator)
                .default_value("768"),
        )
        .arg(
            Arg::with_name("num_threads")
                .short("t")
                .value_name("THREADS")
                .help("Number of render threads")
                .takes_value(true)
                .validator(number_validator)
                .default_value(&default_cpus)
        )
        .arg(
            Arg::with_name("antialiasing")
                .short("a")
                .help("Apply antialiasing")
        );
    let options = app.get_matches();

    let w: u32 = match options.value_of("width").unwrap().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("ERROR: Bad width!");
            return;
        }
    };
    let h: u32 = match options.value_of("height").unwrap().parse() {
        Ok(n) => n,
        Err(_) => {
            println!("ERROR: Bad height!");
            return;
        }
    };

    let options = Options {
        num_threads: options.value_of("num_threads").unwrap().parse().unwrap(),
        width: w,
        height: h,
        background_color: Color::new(0.1, 0.1, 0.5),
        bias: 1e-4,
        max_depth: 5,
        antialiasing: options.is_present("antialiasing"),
    };

    let scene = scene::setup_scene(w, h);

    let imgbuf = system::render(options, scene);

    let ref mut fout = File::create("out.png").unwrap();
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
