fn if_test(){
    let number = 3;
    if number < 5 {
        println!("the number is smaller than 5");
    } else {
        println!("the number is smaller than 5");
    }
}

fn fizz_buzz(n: u32){
    let mut i = 0;
    while i <= n {
        if i % 15 == 0 {
            println!("fizzbuzz");
        } else if i % 5 == 0 {
            println!("buzz");
        } else if i % 3 == 0 {
            println!("fizz");
        } else {
            println!("{}", i);
        }
        i+=1;
    }
}

fn if_return_test(n: u32){
    let x = if n < 10 {
        n
    } else {
        10  //"too large" is not possible. Same return type in all if cases
    };
    println!("{}", x);
}

fn for_test(){
    for i in 1..5 {
        println!("{}", i)
    }
}

fn main() {
    if_test();
    fizz_buzz(15);
    if_return_test(12);
    for_test();
}
