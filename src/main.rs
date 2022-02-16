use std::io;

fn main() {
    println!("*** UTF-8 CONVERTER ***");
    println!("Insert the UTF-8 code to convert:");

    let mut code = String::new();

    io::stdin()
        .read_line(&mut code)
        .expect("Failed to read line.");

    // let converted_code = utf_converter::build_octect(&code);
    println!("You inserted: {}", code);
}
