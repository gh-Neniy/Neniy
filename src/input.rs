use crate::{NeniyError::Io, Result};
use std::{env::Args, path::PathBuf};

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
        return Err(Io("no one path specified".to_string()));
    }
    if output_dir.is_empty() {
        return Err(Io("output directory is not specified".to_string()));
    }

    Ok(())
}

pub fn parse_input() -> Result<(String, Vec<PathBuf>, String)> {
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
