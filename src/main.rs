use neniy::compile;
use neniy::input;
use std::process;

fn main() {
    let (is_check, _, paths, output_dir) = input::parse_input().unwrap_or_else(|error| {
        eprintln!("{}", error.as_error_base());
        process::exit(1);
    });

    if !compile::compile_on_paths(&paths, &output_dir, is_check) {
        process::exit(1);
    }
}
