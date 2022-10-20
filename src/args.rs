use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[clap(author, about, about)]
/// Grasp is a simple cli application. 
/// It runs cli commands from config file 'grasp.json'
pub struct Cli {
    #[command(subcommand)]
    pub action: Action,
}

#[derive(Subcommand, Debug)]
pub enum Action {
    /// Run you tasks from clip.json 
    Run(RunAction),
}

#[derive(Parser, Debug)]
pub struct RunAction {
    /// Path to directory with clip.json
    #[clap(short, long, default_value = "")]
    pub path: String,
}
