use clap::{Arg, Command};
use std::{env, io};

mod register_days;

fn main() -> io::Result<()> {
    let matches = Command::new("Advent of Code")
        .author("Alexander Dimitrov")
        .about("Runs Advent of Code solutions")
        .arg(
            Arg::new("day")
                .help("The day to run (e.g., day01)")
                .required(true)
                .index(1),
        )
        .get_matches();

    let day = matches
        .get_one::<String>("day")
        .expect("Day argument is required");

    let days_map = register_days::register_days();
    match days_map.get(day.as_str()) {
        Some(func) => {
            env::set_current_dir(day.as_str())?;
            func()
        }
        None => Err(io::Error::new(
            io::ErrorKind::Other,
            format!("Day not found `{day}`"),
        )),
    }
}
