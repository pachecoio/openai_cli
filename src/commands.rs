use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Main commands
    #[clap(subcommand)]
    pub commands: CommandTypes
}

#[derive(Debug, Subcommand)]
pub enum CommandTypes {
    /// Text completion commands
    GenerateText(TextCommand),

    /// Image generation commands
    GenerateImage(ImageCommand),
}

#[derive(Parser, Debug)]
pub struct TextCommand {
    #[arg(short, long)]
    pub description: String,

    #[arg(long)]
    pub model: Option<String>,

    #[arg(short, long, default_missing_value = "0.0")]
    pub temperature: Option<f32>,

    #[arg(short, long, default_missing_value = "64")]
    pub max_tokens: Option<u32>,
}

#[derive(Parser, Debug)]
pub struct ImageCommand {
    #[arg(short, long)]
    pub description: String,
}
