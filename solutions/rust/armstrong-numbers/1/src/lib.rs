pub fn is_armstrong_number(num: u32) -> bool {
    // todo!("true if {num} is an armstrong number")
    let num_string: String = num.to_string();
    let n = num_string.len();
    let mut sum = 0;

    for digit in num_string.chars() {
        let dig: i32 = digit.to_string().parse().expect("Not a number");
        sum = dbg!(sum + dig.pow(n as u32));
    }

    sum as u32 == num
}