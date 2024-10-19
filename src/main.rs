fn main() {
    println!("Hello, world!");
    //let ans = is_even(4);
    // println!("Is 10 even? {}", ans);

    // println!("fibonaccini value at position 10: {}", fib2(4));

    let str = String::from("Hello world! I am Abhinandan");
    let lenght = get_str_length(str);
    println!("Length of the str is {}", lenght);
}

fn is_even(num:i32) -> bool {
    return num % 2 == 0
}

fn fib(num: i32) -> i32 {
   if num <= 1 {
    return 0;
   }else if num == 2 {
       return  1;
   }else {
       return fib(num-1) + fib(num - 2);
   }
}

fn fib2(num: i32) -> i32 {
    let mut a = 0;
    let  mut b = 1;

    if(num <= 0){
        return a;
    }
    if num == 1 {
        return  b;
    }

    for i in 1..num-2 {
        let temp =  b;
        b = b + a;
        a = temp;
    }

    return b;
}

fn get_str_length(s: String) -> usize {
 return s.chars().count();
}