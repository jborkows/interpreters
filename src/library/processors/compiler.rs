use std::io::prelude::*;
use std::{fs, io};

use crate::{
    ast::statements::{self, Program, Statement},
    code::{CompilationError, compile},
    evaluator::{define_macros, expand_macros},
    lexers::Lexer,
    object::new_environment,
    parser::Parser,
};
use anyhow::{Context, Result, bail};
use thiserror::Error;
use zip::write::SimpleFileOptions;

#[derive(Error, Debug)]
pub enum FileError {
    #[error("File '{0}' is empty")]
    EmptyFile(String),

    #[error("File '{0}' not found")]
    NotFound(String),

    #[error("File '{path}' has invalid format: {reasons}")]
    InvalidFormat { path: String, reasons: String },

    #[error("Compilation failed: {error:?}")]
    CompilationFailed { error: CompilationError },

    #[error("IO error")]
    Io(#[from] io::Error),

    #[error("Errors: {0:?}")]
    Combined(Vec<FileError>),
    #[error("Zip error: {0}")]
    Zip(#[from] zip::result::ZipError),
}

pub struct Compiler<'a> {
    output_path: &'a str,
    programs: Vec<Program>,
    errors: Vec<FileError>,
}

impl<'a> Compiler<'a> {
    pub fn new(ouput_path: &'a str) -> Self {
        Compiler {
            output_path: ouput_path,
            programs: vec![],
            errors: vec![],
        }
    }

    pub fn process(&mut self, file_path: &str) {
        let content = match read_nonempty_file(file_path) {
            Ok(v) => v,
            Err(e) => {
                eprintln!("While processing {file_path}: {e}");
                self.errors.push(e);
                return;
            }
        };
        let mut parser = Parser::from_string(&content);
        let program = parser.parse_program();
        let errors = parser.errors();
        self.errors.push(FileError::InvalidFormat {
            path: file_path.to_string(),
            reasons: errors.iter().fold("".to_string(), |a, c| a + c),
        });
        if errors.is_empty() {
            self.programs.push(program);
        }
    }

    pub fn compile_program(self) -> Result<(), FileError> {
        let mut errors = self.errors;
        let macro_environemnt = new_environment();
        let statements: Vec<Statement> = self
            .programs
            .iter()
            .flat_map(|p| p.statements.clone())
            .collect();
        if statements.is_empty() {
            return Err(FileError::Combined(errors));
        }
        let program = Program {
            statements: statements,
        };
        let macro_defined = define_macros(program, macro_environemnt.clone());
        let macro_expanded_program = expand_macros(macro_defined, macro_environemnt.clone());

        match compile(macro_expanded_program) {
            Ok(v) => {
                if errors.is_empty() {
                    let path = std::path::Path::new(self.output_path);
                    let file = std::fs::File::create(path).unwrap();

                    let mut zip = zip::ZipWriter::new(file);
                    zip.add_directory("output", SimpleFileOptions::default())?;

                    let options = SimpleFileOptions::default()
                        .compression_method(zip::CompressionMethod::Stored)
                        .unix_permissions(0o755);
                    zip.start_file("output/instructions", options)?;
                    let instruction_bytes: Vec<u8> =
                        v.instructions.bytes().iter().map(|i| i.0).collect();
                    zip.write_all(&instruction_bytes)?;
                    zip.start_file("output/constants", options)?;
                    //TODO: how to store constants (and restore)
                    todo!()
                } else {
                    return Err(FileError::Combined(errors));
                }
            }
            Err(error) => {
                for e in error {
                    errors.push(FileError::CompilationFailed { error: e });
                }
                return Err(FileError::Combined(errors));
            }
        }
    }
}

fn read_nonempty_file(path: &str) -> Result<String, FileError> {
    let contents = fs::read_to_string(path)?;

    if contents.is_empty() {
        return Err(FileError::EmptyFile(path.to_string()));
    }

    Ok(contents)
}
