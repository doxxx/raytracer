extern crate image;
extern crate clap;
extern crate wavefront_obj;
extern crate pbr;
extern crate num_cpus;
extern crate time;
extern crate rand;

mod color;
mod direction;
mod materials;
mod matrix;
mod object;
mod point;
mod sdl;
mod shapes;
mod system;
mod texture;
mod vector;

mod sdl_grammar {
    include!(concat!(env!("OUT_DIR"), "/sdl_grammar.rs"));
}

use std::io::Stdout;
use std::io::prelude::*;
use std::fs::File;

use clap::{App, Arg};
use pbr::ProgressBar;

use color::Color;
use system::Options;
use system::RenderProgress;

fn u16_validator(s: String) -> Result<(), String> {
    if s.parse::<u16>().is_ok() { return Ok(()); }
    Err(String::from("The value must be a positive number."))
}

fn u32_validator(s: String) -> Result<(), String> {
    if s.parse::<u32>().is_ok() { return Ok(()); }
    Err(String::from("The value must be a positive number."))
}

fn usize_validator(s: String) -> Result<(), String> {
    if s.parse::<usize>().is_ok() { return Ok(()); }
    Err(String::from("The value must be a positive number."))
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
                .default_value(&default_cpus)
        )
        .arg(
            Arg::with_name("samples")
                .short("s")
                .help("Number of samples per camera pixel")
                .takes_value(true)
                .validator(u16_validator)
                .default_value("1")
        )
        .arg(
            Arg::with_name("scene")
                .value_name("FILE")
                .help("The file describing the scene to render")
                .required(true)
                .index(1)
        );
    let args = app.get_matches();

    let w: u32 = args.value_of("width").unwrap().parse().expect("ERROR: Bad width!");
    let h: u32 = args.value_of("height").unwrap().parse().expect("ERROR: Bad height!");
    let samples: u16 = args.value_of("samples").unwrap().parse().expect("ERROR: Bad samples!");

    let options = Options {
        num_threads: args.value_of("num_threads").unwrap().parse().unwrap(),
        width: w,
        height: h,
        bias: 1e-4,
        max_depth: 50,
        samples: samples,
    };

    let scene = {
        let mut f = File::open(args.value_of("scene").unwrap()).expect("could not open scene file");
        let mut text = String::new();
        f.read_to_string(&mut text).expect("could not read scene file");
        sdl::parse(&options, &text).expect("could not parse scene file")
    };

    let mut progress = CliRenderProgress::new("out.png");

    system::render(options, scene, &mut progress);
}

struct CliRenderProgress {
    filename: String,
    start_time: time::Tm,
    steady_start_time: time::SteadyTime,
    pb: ProgressBar<Stdout>,
    last_pb_tick: time::SteadyTime,
    last_output_time: time::SteadyTime,
}

impl CliRenderProgress {
    fn new(filename: &str) -> CliRenderProgress {
        CliRenderProgress {
            filename: String::from(filename),
            start_time: time::now(),
            steady_start_time: time::SteadyTime::now(),
            pb: ProgressBar::new(0),
            last_pb_tick: time::SteadyTime::now(),
            last_output_time: time::SteadyTime::now(),
        }
    }
}

impl RenderProgress for CliRenderProgress {
    fn render_started(&mut self, options: &Options) {
        println!("Rendering {}x{}, {} samples per pixel, using {} threads.", options.width, options.height, options.samples, options.num_threads);
        println!("Started at {}", self.start_time.rfc822());

        // Trigger initial progress bar draw
        self.pb.show_tick = true;
        self.pb.total = options.samples as u64;
        self.pb.message("Samples: ");
        self.pb.set(0);
    }

    fn sample_started(&mut self, _options: &Options) {}

    fn row_finished(&mut self, _options: &Options) {
        let now = time::SteadyTime::now();
        if (now - self.last_pb_tick).num_milliseconds() >= 250 {
            self.last_pb_tick = now;
            self.pb.tick();
        }
    }

    fn sample_finished(&mut self, options: &Options, renderbuf: &Vec<Vec<Color>>, num_samples: u16) {
        let now = time::SteadyTime::now();
        if (now - self.last_output_time).num_milliseconds() >= 5000 {
            self.last_output_time = now;

            write_render_result_to_file(&options, &self.filename, &renderbuf, num_samples);
        }

        self.pb.inc();
    }

    fn render_finished(&mut self, options: &Options, renderbuf: &Vec<Vec<Color>>, num_samples: u16) {
        write_render_result_to_file(&options, &self.filename, &renderbuf, num_samples);

        let end_time = time::now();
        let elapsed = time::SteadyTime::now() - self.steady_start_time;

        self.pb.finish_println(&format!("Finished at {} ({})", end_time.rfc822(), format_duration(elapsed)));
    }
}

fn color_to_rgb(v: Color) -> image::Rgb<u8> {
    let r = (v.r * 255.0).min(255.0) as u8;
    let g = (v.g * 255.0).min(255.0) as u8;
    let b = (v.b * 255.0).min(255.0) as u8;
    image::Rgb([r, g, b])
}

fn convert_render_result_to_image(renderbuf: &Vec<Vec<Color>>, num_samples: f64, imgbuf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>) {
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let row = &renderbuf[y as usize];
        let c = (row[x as usize] / num_samples).gamma_2();
        *pixel = color_to_rgb(c);
    }
}

fn write_render_result_to_file(options: &Options, filename: &str, renderbuf: &Vec<Vec<Color>>, current_sample: u16) {
    let mut imgbuf = image::RgbImage::new(options.width, options.height);
    convert_render_result_to_image(&renderbuf, (current_sample + 1) as f64, &mut imgbuf);

    let ref mut fout = File::create(filename).expect("Could not open output file");
    image::ImageRgb8(imgbuf).save(fout, image::PNG).expect("Could not write render result to output file");
}

fn format_duration(mut d: time::Duration) -> String {
    let mut s = String::new();
    let hours = d.num_hours();
    d = d - time::Duration::hours(hours);
    if hours > 0 {
        s += &format!("{}h ", hours);
    }
    let minutes = d.num_minutes();
    d = d - time::Duration::minutes(minutes);
    if minutes > 0 {
        s += &format!("{}m ", minutes);
    }
    let seconds = d.num_seconds();
    d = d - time::Duration::seconds(seconds);
    let milliseconds = d.num_milliseconds();
    if seconds > 0 {
        s += &format!("{}.{:03}s", seconds, milliseconds);
    }
    s
}
