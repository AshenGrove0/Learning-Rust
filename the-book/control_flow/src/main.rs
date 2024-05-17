fn main() {
    let x = 3;
    if x > 5 {
        println!("x is greater than 5");
    } else if x < 3 {
        println!("x is less than 3");
    } else {
        println!("x is between 3 and 5");
    }
    let y = if x == 4 { 12 } else { 13 };
    /*loop { // infinite
        println!("U S A")
    }*/
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter*2 // like a return value for a loop
        }
    };

    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -=1
    }
    let x = [4,2,1,4,5,6,7,76];
    for i in x {
        println!("{}",i);
    }

    for num in (1..4).rev() { // best way
        println!("{}",num);
    }

    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
        
}
