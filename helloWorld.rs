//Ok this is a quick example of rust

fn simple_math(a: i32,b: i32) -> i32{
    let result = a+b;
    return result
}

fn main(){
    println!("Hello world!s");
    println!("Here is the result: {}",simple_math(10, 20));
}