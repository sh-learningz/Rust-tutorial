fn main(){
    let x = {
        let y = 6;
        y * 2
    };
    println!("{x}");

    hello();
    print_nums(12);

    let sum: i32 = sum(6, 6);
    println!("{sum}");

}

fn hello(){
    println!("Hello world");
}

fn print_nums(x: i32){
    println!("your number is {x}")
}

fn sum(x: i32, y:i32) -> i32{
    x + y
}
