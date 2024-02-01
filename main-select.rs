mod project_a;
mod project_b;

use project_a::handle_action_a;
use project_b::handle_action_b;
use structopt::StructOpt;
use dialoguer::{theme::ColorfulTheme, Select};

#[derive(StructOpt, Debug)]
#[structopt(name = "MyApp")]
struct Opt {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let items = vec!["Aについて", "Bについて"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("選択してください")
        .default(0)
        .items(&items)
        .interact_opt()?;

    match selection {
        Some(0) => handle_action_a(),
        Some(1) => handle_action_b(),
        None => println!("選択がキャンセルされました。"),
        _ => {}
    }

    Ok(())
}

