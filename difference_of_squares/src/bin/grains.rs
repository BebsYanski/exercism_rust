fn main() {
    println!("Hello, world!");
    println!("{}", square(1));
    println!("{}", square(2));
    println!("{}", square(3));
    println!("{}", square(4));
    println!("{}", square(16));
    println!("{}", square(32));
    println!("{}", square(64));
    println!("{}", total());
}
pub fn square(s: u32) -> u64 {
    2u64.pow(s - 1)
}

pub fn total() -> u128 {
    2u128.pow(64) - 1
}
