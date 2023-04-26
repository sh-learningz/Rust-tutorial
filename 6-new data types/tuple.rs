fn main(){
    let tup1: (i32, f64, u8) = (500, 6.4, 1);
    let tup2 = (500, 6.4, 1);

    let (x, y, z) = tup2;

    println!("The value of y is: {y}");
}