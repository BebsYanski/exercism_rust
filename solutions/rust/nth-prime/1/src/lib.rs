pub fn nth(n: u32) -> u32 {
    let mut my_vec: Vec<u32> = vec![2, 3, 5, 7, 11, 13];
    let mut num: usize = 14;
    
    while my_vec.len() <= n as usize {
        let mut is_prime = true;  // ← Reset EVERY iteration
        
        for i in 2..=num / 2 {
            if num % i == 0 {
                is_prime = false;
                break;  // ← Exit early if not prime
            }
        }
        
        if is_prime {
            my_vec.push(num as u32);
        }
        
        num += 1;
    }
    
    my_vec[n as usize]
}