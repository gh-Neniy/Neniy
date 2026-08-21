use crate::{
    NeniyError, Result,
    lexic::{self, token::Index},
    synt, trans,
};
use rayon::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

pub fn compile(source_code: &[u8], path: &Path) -> Result<String> {
    let tokens = lexic::parse(source_code)?;
    let nodes = synt::parse(&tokens, source_code)?;

    trans::translate(&nodes, source_code, path)
}

fn compile_on_path(path: &Path, output_dir: &str, is_check: bool) -> Result<()> {
    let content = fs::read_to_string(path)?;
    let source_code = content.as_bytes();

    if source_code.len() > Index::MAX as usize {
        return Err(NeniyError::new_io(
            "file length greater than maximum acceptable length".to_string(),
        ));
    }
    if !source_code.is_empty() && *source_code.last().unwrap() != b'\n' {
        return Err(NeniyError::new_warning(
            "no empty line in the end".to_string(),
        ));
    }

    let result = compile(source_code, path)?;

    if !is_check {
        let mut output_file_path = Path::new(output_dir).join(path);
        output_file_path.set_extension("mcfunction");

        fs::create_dir_all(output_file_path.parent().unwrap())?;
        fs::write(&output_file_path, result)?;
    }

    Ok(())
}

pub fn compile_on_paths(paths: &[PathBuf], output_dir: &str, is_check: bool) -> bool {
    let compile_results: Vec<_> = paths
        .into_par_iter()
        .map(|path| match compile_on_path(path, output_dir, is_check) {
            Ok(()) => Ok(()),
            Err(error) => Err((path, error)),
        })
        .collect();

    let mut json_errors = Vec::new();
    let mut result = true;

    if is_check {
        for compile_result in compile_results {
            if let Err((path, error)) = compile_result {
                result = false;
                json_errors.push(error.as_json(path));
            }
        }

        println!("[{}]", json_errors.join(","));
    } else {
        for compile_result in compile_results {
            if let Err((path, error)) = compile_result {
                result = false;
                eprintln!("{}", error.as_error(path));
            }
        }
    }

    result
}
