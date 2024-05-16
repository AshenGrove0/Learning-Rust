fn main() {
    let guess: u32 = "42".parse().expect("NaN");
    let hex = 0xfe;
    let pi_3dp: f64 = 3.142;
    let tup: (i32, i64, f32) = (200,300,4.21);
    let (x,y,z) = tup;
    println!("{x}");
    let index_1 = tup.1;
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];
    let array: [i64;5] = [1,2,3,4,5];
    let a = [3;5]; // [3,3,3,3,3]
    let first = a[0];

}
