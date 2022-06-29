use std::io::Write;
use structopt::StructOpt;

mod examples;
mod tools;

#[derive(StructOpt, Debug)]
#[structopt(name = "Miden", about = "Miden CLI")]
pub struct Cli {
    #[structopt(subcommand)]
    action: Actions,
}

#[derive(StructOpt, Debug)]
pub enum Actions {
    Example(examples::ExampleOptions),
    Analyze(tools::Analyze),
}

impl Cli {
    pub fn execute(&self) {
        match &self.action {
            Actions::Example(example) => example.execute(),
            Actions::Analyze(analyze) => analyze.execute(),
        }
    }
}

fn main() {
    // configure logging
    env_logger::Builder::new()
        .format(|buf, record| writeln!(buf, "{}", record.args()))
        .filter_level(log::LevelFilter::Debug)
        .init();

    // read command-line args
    let cli = Cli::from_args();

    // execute cli action
    cli.execute();
}
