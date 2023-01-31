use bpaf::Bpaf;
use std::io::{stdin, stdout, Write};
use std::path::PathBuf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
struct Opts {
    #[bpaf(positional)]
    path: Option<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let opts = opts().run();
    let file = match opts.path {
        Some(x) => std::fs::read_to_string(x)?,
        None => std::io::read_to_string(stdin())?,
    };
    let output = shrink_conflicts::run(file)?;
    let _ = stdout().write_all(output.as_bytes());
    Ok(())
}
