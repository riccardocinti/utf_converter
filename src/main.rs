use std::io;

fn main() {
    println!("*** UTF-8 CONVERTER ***");
    println!("Insert the UTF-8 code to convert:");

    let mut code = String::new();

    io::stdin()
        .read_line(&mut code)
        .expect("Failed to read line.");

    let without_prefix = code.trim_end().trim_start_matches("0x");
    let decimal_code = usize::from_str_radix(without_prefix, 16).expect("Conversion failed");
    let converted_code = utf_converter::convert_to_bytes(decimal_code);

    println!("You inserted: {}", code.trim_end());
    println!("The converted bytes result is: {}", converted_code);
}
