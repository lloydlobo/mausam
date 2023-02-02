use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct Cli {
    // #[arg(default_value = "London")]
    pub place: Option<String>,
    // pub coord: Option<Coordinates>,
}
