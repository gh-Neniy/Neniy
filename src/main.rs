use neniy::{
    NeniyError::Io,
    Result,
    lexic::{self, token::Index},
    synt, trans,
};
use rayon::prelude::*;
use std::{
    env::Args,
    fs,
    path::{Path, PathBuf},
    process,
};

fn flag_value(args: &mut Args, name: &str) -> Result<String> {
    match args.next() {
        Some(arg) => Ok(arg),
        None => Err(Io([name, " not found"].concat())),
    }
}

fn parse_flag(
    flag: &str,
    args: &mut Args,
    mc_version: &mut String,
    output_dir: &mut String,
) -> Result<()> {
    match flag {
        "-v" => {
            *mc_version = flag_value(args, "mc_version")?;
        }
        "-d" => {
            *output_dir = flag_value(args, "output_dir")?;
        }

        _ => return Err(Io(["invalid flag ", flag].concat())),
    }

    Ok(())
}

fn parse_arg(arg: &str, paths: &mut Vec<PathBuf>) -> Result<()> {
    let path = PathBuf::from(arg);

    if !path.exists() {
        return Err(Io(["path ", arg, " does not exist"].concat()));
    }
    if !path.is_file() {
        return Err(Io(["path ", arg, " is not a file"].concat()));
    }
    if !path
        .extension()
        .is_some_and(|extension| extension == "neniy")
    {
        return Err(Io(["file ", arg, " has not \".neniy\" extension"].concat()));
    }

    paths.push(path);
    Ok(())
}

fn validate(_: &str, paths: &[PathBuf], output_dir: &str) -> Result<()> {
    // if mc_version.is_empty() {
    //     return Err(Io("minecraft version in not specified".to_string()));
    // }
    if paths.is_empty() {
        return Err(Io("No one path specified".to_string()));
    }
    if output_dir.is_empty() {
        return Err(Io("output directory is not specified".to_string()));
    }

    Ok(())
}

fn parse_input() -> Result<(String, Vec<PathBuf>, String)> {
    let mut args = std::env::args();
    args.next();

    let mut mc_version = String::new();
    let mut paths = Vec::new();
    let mut output_dir = String::new();

    while let Some(arg) = args.next() {
        if arg.starts_with('-') {
            parse_flag(&arg, &mut args, &mut mc_version, &mut output_dir)?;
        } else {
            parse_arg(&arg, &mut paths)?;
        }
    }

    validate(&mc_version, &paths, &output_dir)?;

    Ok((mc_version, paths, output_dir))
}

fn compile(source_code: &[u8], path: &Path) -> Result<String> {
    let tokens = lexic::parse(source_code)?;
    let nodes = synt::parse(&tokens, source_code)?;

    trans::translate(&nodes, source_code, path)
}

fn compile_on_path(path: &Path, output_dir: &str) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let source_code = content.as_bytes();

    if source_code.len() > Index::MAX as usize {
        return Err(Io(
            "file length greater than maximum acceptable length".to_string()
        ));
    }
    if !source_code.is_empty() && *source_code.last().unwrap() != b'\n' {
        eprintln!(
            "In file {}: Warning - no empty line in the end",
            path.display()
        );
    }

    let result = compile(source_code, path)?;

    let output_file_path = PathBuf::from(output_dir)
        .join(path)
        .with_extension("mcfunction");

    fs::create_dir_all(output_file_path.parent().unwrap())?;
    fs::write(&output_file_path, result)?;

    Ok(())
}

fn compile_on_paths(paths: &[PathBuf], output_dir: &str) -> bool {
    paths
        .into_par_iter()
        .map(|path| match compile_on_path(path, output_dir) {
            Ok(()) => true,
            Err(error) => {
                eprintln!("in file {}: {}", path.display(), error);
                false
            }
        })
        .reduce_with(|a, b| a & b)
        .unwrap()
}

fn main() {
    let (_, paths, output_dir) = match parse_input() {
        Ok(result) => result,
        Err(error) => {
            eprintln!("{error}");
            process::exit(1);
        }
    };

    if !compile_on_paths(&paths, &output_dir) {
        process::exit(1);
    }
}
