mod codegen;
mod utils;

use eyre::Result;

fn main() -> Result<()> {
    codegen::run()?;

    Ok(())
}
