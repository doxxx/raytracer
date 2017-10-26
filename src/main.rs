extern crate image;
extern crate clap;
extern crate wavefront_obj;
extern crate pbr;
extern crate num_cpus;
extern crate time;
extern crate rand;

mod color;
mod direction;
mod kdtree;
mod lights;
mod materials;
mod matrix;
mod object;
mod photon;
mod point;
mod sdl;
mod shaders;
mod shapes;
mod system;
mod texture;
mod vector;

mod sdl_grammar {
    include!(concat!(env!("OUT_DIR"), "/sdl_grammar.rs"));
}

use std::fs::File;
use std::io::prelude::*;

use clap::{App, Arg};

use color::Color;
use system::Options;

fn u32_validator(s: String) -> Result<(), String> {
    if s.parse::<u32>().is_ok() {
        return Ok(());
    }
    Err(String::from("The value must be a number."))
}

fn usize_validator(s: String) -> Result<(), String> {
    if s.parse::<usize>().is_ok() {
        return Ok(());
    }
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
                .validator(u32_validator)
                .default_value("1024"),
        )
        .arg(
            Arg::with_name("height")
                .short("h")
                .value_name("HEIGHT")
                .help("Image height")
                .takes_value(true)
                .validator(u32_validator)
                .default_value("768"),
        )
        .arg(
            Arg::with_name("num_threads")
                .short("t")
                .value_name("THREADS")
                .help("Number of render threads")
                .takes_value(true)
                .validator(usize_validator)
                .default_value(&default_cpus),
        )
        .arg(Arg::with_name("antialiasing").short("a").help(
            "Apply antialiasing",
        ))
        .arg(
            Arg::with_name("scene")
                .value_name("FILE")
                .help("The file describing the scene to render")
                .required(true)
                .index(1),
        );
    let args = app.get_matches();

    let w: u32 = args.value_of("width").unwrap().parse().expect(
        "ERROR: Bad width!",
    );
    let h: u32 = args.value_of("height").unwrap().parse().expect(
        "ERROR: Bad height!",
    );

    let options = Options {
        num_threads: args.value_of("num_threads").unwrap().parse().unwrap(),
        width: w,
        height: h,
        background_color: Color::black(),
        bias: 1e-4,
        max_depth: 5,
        antialiasing: args.is_present("antialiasing"),
    };

    let scene = {
        let mut f = File::open(args.value_of("scene").unwrap()).expect("could not open scene file");
        let mut text = String::new();
        f.read_to_string(&mut text).expect(
            "could not read scene file",
        );
        sdl::parse(&text).expect("could not parse scene file")
    };

    let imgbuf = system::render(options, scene);

    let ref mut fout = File::create("out.png").unwrap();
    let _ = image::ImageRgb8(imgbuf).save(fout, image::PNG);
}
