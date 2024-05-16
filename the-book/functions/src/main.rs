fn main() {
    println!("Hello, world!");
    not_main();
    multiply(5,4);
    let x = {
        let y = 2;
        y+1
    }
    let mut num = five();
    increment(num);
}

fn not_main() {
    println!("I'm not main!");
}

fn multiply(num1:i32, num2:i32) {
    println!("{num1}*{num2}={}",num1*num2);
}

fn five() -> i32 {
    5 // no semicolon as we want to return it
}

fn increment(count: i32) -> i32 {
    count+1
}