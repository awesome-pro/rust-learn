fn main() {
    println!("Hello, world!");
    let ans = is_even(11);
    println!("Is 10 even? {}", ans);
}

fn is_even(num:i32) -> bool {
    return num % 2 == 0
}