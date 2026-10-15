use taste_engine::add;
use taste_engine::app::run;
fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let num = add(1, 1);
    println!("{num}");

    run()?;
    Ok(())
}
