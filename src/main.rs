extern crate byteorder;
extern crate docopt;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate libawl;

use std::process;
use docopt::Docopt;

mod serve;
mod connect;


macro_rules! command_list {
    () => (
"
    serve         Act as a rendevous server for connecting multiple peers
    connect       Connect to a peer the rendevous implementation
"
    )
}

static USAGE: &'static str = concat!("
Usage:
    awl <command> [<args>...]

Options:
    --list          List all commands
    <command> -h    Displays command help message
    --version       Displays version information
Commands:", command_list!());

#[derive(Deserialize)]
struct Args {
    arg_command: Option<Command>,
    flag_list: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.options_first(true)
                  .version(Some("0.1".to_string()))
                  .deserialize())
        .unwrap_or_else(|e| e.exit());

    match args.arg_command {
        None => println!("We're done here"),
        Some(cmd) => {
            match cmd.run() {
                _ => process::exit(0),
            }
        }
    }
}

#[derive(Debug, Deserialize)]
enum Command {
    Serve,
    Connect,
}

impl Command {
    fn run(self) -> () {
        match self {
            Command::Serve => serve::serve(),
            Command::Connect => connect::connect(),
        }
    }
}
