fn main(){
    let a = [1, 2, 3, 4, 5];

    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let b : [i32; 5] = [1, 2, 3, 4, 5];

    let mut num = b[2];
    println!("{num}");

    let c = [12; 12];

    num = c[0];
    println!("{num}");
}