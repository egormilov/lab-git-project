use std::io;

pub fn multiply_numbers(a: u32, b: u32) -> u32{
    a * b
}

pub fn add_numbers(a: u32, b: u32) -> u32{
    a + b
}

pub fn book_name() -> String{
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    guess
}