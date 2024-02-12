use app::init_host;
use anyhow::Context;


fn main() -> anyhow::Result<()> {
    init_host().context("Failed to initialize host")?;
    Ok(())
}
