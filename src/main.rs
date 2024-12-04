use std::env;

pub mod day1;
pub mod day2;

fn main() {
    let mut args = env::args().skip_while(|arg| arg != "-d" && arg != "--day");
    match args
        .nth(1)
        .expect("expected `-d <day>` or `--day <day>` arg")
        .as_str()
    {
        "1" => day1::run(),
        "2" => day2::run(),
        _ => panic!("expected a day number"),
    }
}
