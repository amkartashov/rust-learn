use std::io;

fn main() {
    let mut input = String::new();
    let n :i32 = loop {
        println!("Enter number:");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");
        match input.trim().parse() {
            Ok(num) => break num,
            Err(_) => continue,
        }
    };

    println!("Fibonacci({}) = {}", n, fib(n));
}

fn fib(n: i32) -> i32 {
    match n {
        0 => 0,
        1 => 1,
        //n => fib(n-2) + fib(n-1),
        n => {
            let (mut fi_2, mut fi_1, mut fi) = (0, 1, 1);
            for _ in 2..=n {
                fi = fi_2 + fi_1;
                fi_2 = fi_1;
                fi_1 = fi;
            }
            fi
        },
    }
}
