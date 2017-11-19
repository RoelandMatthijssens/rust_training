fn lets(){
    let mut x = 5;
    x = x + 1;
    println!("The value of x is {}", x);
    let y = 5;
    let y = y + 1;
    println!("The value of y is {}", y);
}

fn tuples(){
    let tup: (u32, f64, u8) = (1, 2.0, 3);
    let (x, y, z) = tup;
    println!("{}, {}, {}", x, y, z);
}

fn arrays(){
    let a:[u32;3] = [4, 5, 6];
    let x = a[0];
    let y = a[1];
    let z = a[2];
    println!("{}, {}, {}", x, y, z);
}

fn print(x:u32){
    println!("{}", x);
}

fn fib(n:u32) -> u32{
    match n {
        0   => 0,
        1   => 1,
        _   => fib(n-1)+fib(n-2)
    }
}

fn main() {
    lets();
    tuples();
    arrays();
    print(4);

    let x = fib(10);
    println!("{}", x);
}
