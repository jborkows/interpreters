use anyhow::{Result, bail};
use clap::Parser;
use interpreter::Compiler;

#[derive(Parser)]
#[command(name = "compiler")]
#[command(about = "Compiles files into intermediate file", long_about = None)]
struct Args {
    #[arg(required = true)]
    files: Vec<String>,

    #[arg(required = true)]
    output: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let mut compiler = Compiler::new(&args.output);
    for file_path in &args.files {
        compiler.process(file_path);
    }

    match compiler.compile_program() {
        Ok(_) => Ok(()),
        Err(error) => {
            eprint!("{error}");
            bail!("Error while compiling")
        }
    }
}
