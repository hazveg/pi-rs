use std::f64::consts::PI;

#[derive(PartialEq)]
enum Mode {
    Exit,
    Simple,
    Verbose,
}

fn parse_args(command: &str) -> Mode {
    match command {
        "--help" | "-h" | "help" => {
            println!("--help / -h / help\t\tPrint this help");
            println!("--verbose / -v / verbose\tPrint number of iterations, calculated pi and the difference from std::f64::consts::PI");
            println!("--simple / -s /simple\t\tPrint just the calculated pi");
            Mode::Exit
        },
        "--verbose" | "-v" | "verbose" => Mode::Verbose,
        "--simple" | "-s" | "simple" => Mode::Simple,
        _ => {
            println!("unclear arguments provided, outputting simple");
            Mode::Simple
        }
    }
}

fn sqr(number: f64) -> f64 {
    number * number
}

fn b(radius: f64, a: f64) -> f64 {
    (sqr(radius) - sqr(a / 2.0)).sqrt()
}

fn main() {
    let mut args = std::env::args();

    let command = args.nth(1).unwrap_or("".to_string());

    let mode = parse_args(command.as_str());

    let mut a = 1.0;
    let radius = 1.0;
    let mut sides = 6.0;

    for i in 1..=25 {
        sides *= 2.0;
        a = (sqr(a / 2.0) + sqr(radius - b(radius, a))).sqrt();
        let our_pi = (sides * a) / (2.0 * radius);

        match mode {
            Mode::Exit => break,
            Mode::Simple => println!("{our_pi}"),
            Mode::Verbose => println!("iter {i}: {our_pi} diff: {}", (PI - our_pi)),
        }
    }
}
