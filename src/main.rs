use clap::Parser;
use convert_rs::convert_temperature;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(name = "convert-rs", version, about = "Unit conversion CLI tool")]
struct Cli {
    /// Temperature value to convert
    #[clap(value_parser)]
    value: f64,

    /// Unit to convert from
    #[clap(value_parser)]
    from: String,

    /// Unit to convert to
    #[clap(value_parser)]
    to: String,
}


fn main() {
    let cli = Cli::parse();

    let value = cli.value;
    let from = cli.from.to_lowercase();
    let to = cli.to.to_lowercase();

    match convert_temperature(value, &from, &to) {
        Some(result) => println!("Converted value: {} {} = {} {}", value, from, result, to),
        None => println!("Invalid conversion from {} to {}", from, to),
    }

}