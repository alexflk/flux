use std::env;

use fluxer::Iteration;
use getopts::Options;

const RESCALE: u32 = 2;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} OUTPUT [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt(
        "",
        "height",
        "height of the output image in pixels. Defaults to 1080",
        "HEIGHT",
    );
    opts.optopt(
        "",
        "width",
        "width of the output image in pixels. Defaults to 1920",
        "WIDTH",
    );
    opts.optopt(
        "",
        "density",
        "density of points in final image. Defaults to 0.1",
        "DENSITY",
    );
    opts.optflag(
        "i",
        "invert",
        "inverts colors of the final image. Defaults to false",
    );
    opts.optflag(
        "s",
        "smooth",
        "smooth final image by super-sampling. Defaults to false",
    );
    opts.optflag("h", "help", "print this help menu");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };

    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }

    let output = if !matches.free.is_empty() {
        matches.free[0].clone()
    } else {
        print_usage(&program, opts);
        return;
    };

    let height = matches.opt_get_default("height", 1080).unwrap();
    let width = matches.opt_get_default("width", 1920).unwrap();
    let density = matches.opt_get_default("density", 0.1).unwrap();
    let invert = matches.opt_present("i");
    let smooth = matches.opt_present("s");

    let max_iterations = ((height * RESCALE * width * RESCALE) as f32 * density).round() as u32;

    let iter = Iteration::new(height * RESCALE, width * RESCALE, max_iterations);
    let mut imgbuf = iter.generate();

    if invert {
        image::imageops::colorops::invert(&mut imgbuf);
    }

    if smooth {
        imgbuf = image::imageops::resize(&imgbuf, width, height, image::FilterType::Triangle);
    }

    imgbuf.save(output).unwrap();
}
