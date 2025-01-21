mod utils;

fn main() {
    let a: u32 = 32;
    let b: u32 = 8;
    let sum = utils::add_numbers(a, b);
    println!("{}", sum.to_string());
}