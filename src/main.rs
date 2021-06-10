use std::io::Write;

mod core;
use crate::core::util::ExitStatus;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 5 {
        writeln!(std::io::stderr(), "usage : ./mandelbrot <filename> <pixels> <top_right_angle> <bottom_left_angle>").unwrap();
        // writeln!(std::io::stderr(), "example : ./mandelbrot mandelbrot.png 1000x750 -1.20,0.35 -1,0.20").unwrap();
        writeln!(std::io::stderr(), "example : ./mandelbrot mandelbrot.png 4000x3000 -1.20,0.35 -1,0.20").unwrap();
        std::process::exit(ExitStatus::EXIT_FAILURE);
    }

    let surface = core::argument::parse_pair(&args[2], 'x').expect("Dimension image error");
    let top_left_angle = core::argument::parse_complex(&args[3]).expect("Top left rectangle coordonates error");
    let right_bottom_angle = core::argument::parse_complex(&args[4]).expect("Bottom right rectangle coordonates error");
    let mut pixels = vec![0; surface.0 * surface.1];

    core::display::exec_render(num_cpus::get(), &mut pixels, surface, top_left_angle, right_bottom_angle);
    core::write_img(&args[1], &pixels, surface).expect("Failed to write PNG file");

    std::process::exit(ExitStatus::EXIT_SUCCESS);
}
