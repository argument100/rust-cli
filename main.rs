use structopt::StructOpt;
use dialoguer::Input;

#[derive(StructOpt, Debug)]
#[structopt(name = "MyApp")]
struct Opt {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let name: String = Input::new()
        .with_prompt("あなたの名前は何ですか？")
        .interact_text()?;

    println!("こんにちは、{}さん！", name);
    Ok(())
}
