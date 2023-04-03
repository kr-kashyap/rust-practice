fn main() {
    println!("Hello, world!");
    println!("conditional_return(1,4) : {}",conditional_return(1,4));
    println!("10 : {}",return_10());
    println!("conditional_return(12,4) : {}",conditional_return(12,4));
}

fn return_10() -> i32{
    10
}

fn conditional_return(x:i32, y:i32) -> i32{
    let result = x + y;
    if result > 10 {
        return  result - 10
    }
    result
}