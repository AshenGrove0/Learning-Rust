fn main() {
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6;
    println!("The value of x is {x}");
    const HOUR_IN_SECONDS: u32 = 60*60;
    println!("24 hours in seconds is {}",HOUR_IN_SECONDS*24);
    let y = 5;
    println!("y is {y}");
    let y = y + 1;
    println!("y is {y}");
    {
        let y = y * 4;
        println!("y is {y} in the inner scope");
    }
    println!("y is {y} in the outer scope");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("Spaces: {spaces}")
}