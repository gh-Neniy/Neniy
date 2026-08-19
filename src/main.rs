use neniy::compile;
use neniy::input;
use std::process;

fn main() {
    let (_, paths, output_dir) = input::parse_input().unwrap_or_else(|error| {
        eprintln!("{error}");
        process::exit(1);
    });

    if !compile::compile_on_paths(&paths, &output_dir) {
        process::exit(1);
    }
}
