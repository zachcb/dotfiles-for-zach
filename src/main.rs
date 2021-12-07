use dialoguer::MultiSelect;
use console::Term;

fn main() -> std::io::Result<()> {
    let selections = MultiSelect::new()
        .item("Option A")
        .item("Option B")
        .interact_on(&Term::stderr())?;

    println!("User selected options at indices {:?}", selections);

    Ok(())
}