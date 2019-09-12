mod draw;

use std::fs;
use std::fs::File;
use std::io::ErrorKind;
use std::io;

fn main() {
    match File::open("coffee_records.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("coffee_records.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };

    loop {
        let contents = read_file_and_draw();
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Unable to read line");
        if input.trim().to_uppercase() == "R" {
                let registered_coffee = match contents.parse::<i32>() {
                    Ok(registered_coffee) => registered_coffee + 1,
                    Err(_) => 1
                };
                let contents = registered_coffee.to_string();
                fs::write("coffee_records.txt",contents).expect("Unable to write file.");
        } else if input.trim().to_uppercase() == "Q" {
            break
        } else {
            println!("No such option is available.");
        }
    }
}

fn read_file_and_draw() -> String {
    let contents = match fs::read_to_string("coffee_records.txt") {
        Ok(contents) => contents,
        Err(_) => "0".to_string()
    };
    draw::draw(contents.to_string());
    contents.to_string()
}
