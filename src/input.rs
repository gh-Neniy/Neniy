use crate::{NeniyError, Result};
use std::{env::Args, path::PathBuf};

fn flag_value(args: &mut Args, name: &str) -> Result<String> {
    match args.next() {
        Some(arg) => Ok(arg),
        None => Err(NeniyError::new_io([name, " not found"].concat())),
    }
}

fn parse_flag(
    flag: &str,
    args: &mut Args,
    is_check: &mut bool,
    mc_version: &mut String,
    output_dir: &mut String,
) -> Result<()> {
    match flag {
        "--check" => *is_check = true,
        "-v" => {
            *mc_version = flag_value(args, "mc_version")?;
        }
        "-d" => {
            *output_dir = flag_value(args, "output_dir")?;
        }

        _ => return Err(NeniyError::new_io(["invalid flag ", flag].concat())),
    }

    Ok(())
}

fn parse_arg(arg: &str, paths: &mut Vec<PathBuf>) -> Result<()> {
    let path = PathBuf::from(arg);

    if !path.exists() {
        return Err(NeniyError::new_io(
            ["path ", arg, " does not exist"].concat(),
        ));
    }
    if !path.is_file() {
        return Err(NeniyError::new_io(
            ["path ", arg, " is not a file"].concat(),
        ));
    }
    if !path
        .extension()
        .is_some_and(|extension| extension == "neniy")
    {
        return Err(NeniyError::new_io(
            ["file ", arg, " has not \".neniy\" extension"].concat(),
        ));
    }

    paths.push(path);
    Ok(())
}

fn validate(mc_version: &str, paths: &[PathBuf], output_dir: &str) -> Result<()> {
    if mc_version.is_empty() {
        return Err(NeniyError::new_io(
            "minecraft version in not specified".to_string(),
        ));
    }
    if paths.is_empty() {
        return Err(NeniyError::new_io("no one path specified".to_string()));
    }
    if output_dir.is_empty() {
        return Err(NeniyError::new_io(
            "output directory is not specified".to_string(),
        ));
    }

    Ok(())
}

pub fn parse_input() -> Result<(bool, String, Vec<PathBuf>, String)> {
    let mut args = std::env::args();
    args.next();

    let mut is_check = false;
    let mut mc_version = String::new();
    let mut paths = Vec::new();
    let mut output_dir = String::new();

    while let Some(arg) = args.next() {
        if arg.starts_with('-') {
            parse_flag(
                &arg,
                &mut args,
                &mut is_check,
                &mut mc_version,
                &mut output_dir,
            )?;
        } else {
            parse_arg(&arg, &mut paths)?;
        }
    }

    validate(&mc_version, &paths, &output_dir)?;

    Ok((is_check, mc_version, paths, output_dir))
}
