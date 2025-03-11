extern crate clap;
extern crate image;
extern crate num_complex;
extern crate num_cpus;
extern crate num_traits;
extern crate pbr;
extern crate rand;
extern crate rayon;
extern crate time;
extern crate wavefront_obj;

#[cfg(test)]
#[macro_use]
mod test_utils;

mod algebra;
mod color;
mod direction;
mod materials;
mod matrix;
mod object;
mod point;
mod sdl;
mod sdl_grammar;
mod shapes;
mod system;
mod texture;
mod vector;

use std::fs::File;
use std::io::Stdout;
use std::io::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::thread::JoinHandle;
use std::thread::sleep;
use std::thread::spawn;
use std::time::Duration;

use clap::Parser;
use pbr::ProgressBar;
use rayon::ThreadPoolBuilder;

use crate::color::Color;
use crate::system::Options;
use crate::system::RenderProgress;

#[derive(Parser)]
#[command(
    version = "0.1.0",
    author = "Gordon Tyler <gordon@doxxx.net>",
    about = "Simple ray tracer"
)]
struct CommandLineOptions {
    /// Image width
    #[arg(long, default_value = "1024", value_parser = clap::value_parser!(u32).range(1..))]
    width: u32,

    /// Image height
    #[arg(long, default_value = "768", value_parser = clap::value_parser!(u32).range(1..))]
    height: u32,

    /// Number of render threads
    #[arg(short('t'), long, value_parser = clap::value_parser!(usize))]
    threads: Option<usize>,

    /// Number of samples per camera pixel
    #[arg(short('s'), long, default_value = "1", value_parser = clap::value_parser!(u16).range(1..))]
    samples: u16,

    /// The file describing the scene to render
    #[arg(required = true)]
    scene: String,
}

fn main() {
    let opts: CommandLineOptions = CommandLineOptions::parse();

    let rendering_options = Options {
        num_threads: opts.threads.unwrap_or_else(num_cpus::get),
        width: opts.width,
        height: opts.height,
        bias: 1e-4,
        max_depth: 50,
        samples: opts.samples,
    };

    let scene = {
        let mut f = File::open(&opts.scene).expect("could not open scene file");
        let mut text = String::new();
        f.read_to_string(&mut text).expect("could not read scene file");
        sdl::parse(&rendering_options, &text).expect("could not parse scene file")
    };

    ThreadPoolBuilder::new()
        .num_threads(rendering_options.num_threads)
        .build_global()
        .expect("could not configure threadpool");

    let mut progress = Arc::new(Mutex::new(CliRenderProgress::new("out.png")));

    let (stop_ticker, progress_ticker_handle) = spawn_progress_ticker(&progress);

    system::render(rendering_options, scene, &mut progress);

    stop_ticker.store(true, Ordering::Relaxed);
    progress_ticker_handle.join().unwrap();
}

fn spawn_progress_ticker(progress: &Arc<Mutex<CliRenderProgress>>) -> (Arc<AtomicBool>, JoinHandle<()>) {
    let stop = Arc::new(AtomicBool::new(false));
    let thread_handle = {
        let step = stop.clone();
        let progress = progress.clone();
        spawn(move || {
            loop {
                if step.load(Ordering::Relaxed) {
                    break;
                }
                {
                    let mut progress = progress.lock().unwrap();
                    progress.tick();
                }
                sleep(Duration::from_millis(250));
            }
        })
    };
    (stop, thread_handle)
}

struct CliRenderProgress {
    filename: String,
    start_time: time::Tm,
    steady_start_time: time::SteadyTime,
    pb: ProgressBar<Stdout>,
    last_output_time: time::SteadyTime,
    num_samples: u16,
}

impl CliRenderProgress {
    fn new(filename: &str) -> CliRenderProgress {
        CliRenderProgress {
            filename: String::from(filename),
            start_time: time::now(),
            steady_start_time: time::SteadyTime::now(),
            pb: ProgressBar::new(0),
            last_output_time: time::SteadyTime::now(),
            num_samples: 0,
        }
    }

    fn tick(&mut self) {
        self.pb.tick();
    }
}

impl RenderProgress for CliRenderProgress {
    fn render_started(&mut self, options: &Options) {
        println!(
            "Rendering {}x{}, {} samples per pixel, using {} threads.",
            options.width, options.height, options.samples, options.num_threads
        );
        println!("Started at {}", self.start_time.rfc822());

        // Trigger initial progress bar draw
        self.pb.show_tick = true;
        self.pb.total = options.samples as u64;
        self.pb.message("Samples: ");
        self.pb.set(0);
    }

    fn sample_finished(&mut self, options: &Options, renderbuf: &Vec<Vec<Color>>) {
        self.num_samples += 1;

        let now = time::SteadyTime::now();
        if (now - self.last_output_time).num_milliseconds() >= 5000 {
            self.last_output_time = now;

            write_render_result_to_file(&options, &self.filename, &renderbuf, self.num_samples);
        }

        self.pb.inc();
    }

    fn render_finished(&mut self, options: &Options, renderbuf: &Vec<Vec<Color>>) {
        write_render_result_to_file(&options, &self.filename, &renderbuf, self.num_samples);

        let end_time = time::now();
        let elapsed = time::SteadyTime::now() - self.steady_start_time;

        self.pb.finish_println(&format!(
            "Finished at {} ({})",
            end_time.rfc822(),
            format_duration(elapsed)
        ));
    }
}

fn color_to_rgb(v: Color) -> image::Rgb<u8> {
    let r = (v.r * 255.0).min(255.0) as u8;
    let g = (v.g * 255.0).min(255.0) as u8;
    let b = (v.b * 255.0).min(255.0) as u8;
    image::Rgb([r, g, b])
}

fn convert_render_result_to_image(
    renderbuf: &Vec<Vec<Color>>,
    num_samples: f64,
    imgbuf: &mut image::ImageBuffer<image::Rgb<u8>, Vec<u8>>,
) {
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
    image::ImageRgb8(imgbuf)
        .save(fout, image::PNG)
        .expect("Could not write render result to output file");
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
