mod crud;
mod db;
fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    Ok(())
}
