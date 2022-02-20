use ansi_term::Color::RGB;
use clap::{Arg, Command};
use rust_http_client::command::Command as HttpCommand;
use rust_http_client::presenter::Presenter;
use std::time;

#[tokio::main]
async fn main() {
    let matches = Command::new("rhc")
        .propagate_version(true)
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("get").about("Sends a GET request"))
        .subcommand(
            Command::new("post")
                .about("Sends a POST request with a body")
                .arg(
                    Arg::new("body")
                        .short('b')
                        .long("body")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .arg(
            Arg::new("address")
                .short('a')
                .long("address")
                .takes_value(true),
        )
        .arg(Arg::new("colorize").long("color").default_value("true"))
        .get_matches();

    let address = matches.value_of("address").unwrap();
    let address = if address.starts_with("http") {
        address.to_string()
    } else {
        format!("http://{}", address)
    };
    let address = address.as_str();

    let use_color = matches
        .value_of("colorize")
        .unwrap()
        .parse::<bool>()
        .unwrap_or(true);

    let command = match matches.subcommand() {
        Some(("get", _)) => HttpCommand::new(address).unwrap(), 
        Some(("post", matches)) => HttpCommand::new(address).unwrap().body(matches.value_of("body").unwrap_or("").to_string()),
        _ => unreachable!("Exhausted list of subcommands and subcommand_required prevents `None`")
    };

    let now = time::Instant::now();
    let maybe_response = command.send().await;
    let elapsed = now.elapsed();
    let elapsed = (elapsed.as_secs() as f64
        + f64::from(elapsed.subsec_nanos()) * 1e-9 * 10000f64)
        .trunc()
        / 10000.0;

    println!(
        "{}s",
        if use_color {
            if maybe_response.is_ok() {
                RGB(32, 227, 64).paint(elapsed.to_string()).to_string()
            } else {
                RGB(227, 4, 32).paint(elapsed.to_string()).to_string()
            }
        } else {
            elapsed.to_string()
        }
    );

    let presenter = Presenter::from(maybe_response).colorize(use_color);

    print!("{}", presenter);
}
