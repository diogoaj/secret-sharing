use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    Split(SplitArgs),
    Combine(CombineArgs),
}

#[derive(Args, Debug)]
pub struct SplitArgs {
    #[arg(short)]
    pub threshold: u8,
    #[arg(short)]
    pub number: u8,
}

#[derive(Args, Debug)]
pub struct CombineArgs {
    #[arg(short)]
    pub threshold: u8,
}

impl Cli {
    pub fn get_args() -> Cli {
        Cli::parse()
    }
}
