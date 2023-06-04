use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// executes string stat
    #[arg(short, value_name = "stat")]
    exec: Option<Vec<String>>,

    /// "requires" mod
    #[arg(short, value_name = "mod")]
    link: Option<Vec<String>>,

    /// enters interactive mode after running script
    #[arg(short)]
    interactive: bool,

    script: Option<String>,

    args: Option<Vec<String>>,
}

fn main() {
    let _cli = Cli::parse();
}
