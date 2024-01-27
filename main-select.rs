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
        Some(0) => println!("Aについての情報を表示します。"),
        Some(1) => println!("Bについての情報を表示します。"),
        None => println!("選択がキャンセルされました。"),
        _ => {}
    }

    Ok(())
}

