use std::io;
use clap::Parser;

#[derive(Debug, Parser)]
struct Args {
    #[arg(value_name = "CODE", help = "UTF-8 code to convert")]
    code: String,
}

fn main() {
    if let Err(e) = run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn run(args: Args) -> io::Result<()> {
    let code = args.code;

    let without_prefix = &code.trim_end().trim_start_matches("0x");
    let decimal_code = usize::from_str_radix(without_prefix, 16).expect("Conversion failed");
    let converted_code = utf_converter::convert_to_bytes(decimal_code);

    println!("You inserted: {}", code.trim_end());
    println!("The converted bytes result is: {}", converted_code);

    Ok(())
}