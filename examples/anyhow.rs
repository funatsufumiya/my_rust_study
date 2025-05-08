use anyhow::{anyhow, Result};

fn raise_error(b: bool) -> Result<()> {
    if b {
        return Err(anyhow!("error occured!"));
    }

    Ok(())
}

fn main() -> Result<()> {
    raise_error(true)?;
    Ok(())
}