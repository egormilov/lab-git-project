mod utils;

fn main() {
    let a: u32 = 32;
    let b: u32 = 8;
    let mul = utils::multiply_numbers(a, b);
    println!("{}", mul.to_string());
}