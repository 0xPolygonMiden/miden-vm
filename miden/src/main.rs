mod cli;
pub mod examples;
pub mod tools;

fn main() {
    match cli::execute() {
        Ok(_) => {}
        Err(err) => println!("{}", err),
    }
}
