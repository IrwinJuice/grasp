use args::{Action, Cli};
use clap::Parser;
use task::Task;

mod args;
mod task;

fn main() -> std::io::Result<()> {
    let args = Cli::parse();

    match &args.action {
        Action::Run(run) => {
            Task::run(run.path.as_str())?;
        }
    }
    Ok(())
}
