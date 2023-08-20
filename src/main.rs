use jisort::prelude::*;

fn main() -> Result<(), Error> {
    let config = Config::from_cli();

    jisort::run(config)
}
