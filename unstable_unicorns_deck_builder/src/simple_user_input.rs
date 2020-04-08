use std::io::{self, Write};

pub fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    match io::stdout().flush() {
        Ok(_flush_successful) => {},
        Err(e) => println!("Error on stdout flush: {}", e),
    }

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_goes_into_input_above) => {},
        Err(e) => println!("Error on read line: {}", e),
    }
    input.trim().to_string()
}
