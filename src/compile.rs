use crate::{
    NeniyError::Io,
    Result,
    lexic::{self, token::Index},
    synt, trans,
};
use rayon::prelude::*;
use std::{
    fs,
    path::{Path, PathBuf},
};

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

    let mut output_file_path = Path::new(output_dir).join(path);
    output_file_path.set_extension("mcfunction");

    fs::create_dir_all(output_file_path.parent().unwrap())?;
    fs::write(&output_file_path, result)?;

    Ok(())
}

pub fn compile_on_paths(paths: &[PathBuf], output_dir: &str) -> bool {
    paths
        .into_par_iter()
        .map(|path| match compile_on_path(path, output_dir) {
            Ok(()) => true,
            Err(error) => {
                eprintln!("In file {}: {}", path.display(), error);
                false
            }
        })
        .reduce_with(|a, b| a & b)
        .unwrap()
}
