use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Applies .
    Fill(FillArgs),
}

#[derive(Args)]
pub struct FillArgs {
    /// The path of the file
    pub template_file: String,
    ///
    #[arg(short, long)]
    pub input_config: Option<String>,
    ///
    #[arg(short, long)]
    pub output: Option<String>,
}
