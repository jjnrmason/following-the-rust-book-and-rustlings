fn main() {
    for i in 1..10 {
        println!("{}", fib(i));
    }    

    for i in 1..10 {
        println!("{}", fib_rec(i));
    }
}

fn fib(n: u32) -> u32 {
    if n == 1 {
        return 1;
    }

    let mut a = 0;
    let mut b = 1;
    for _ in 2..n+1 {
        let c = a + b;
        a = b;
        b = c;
    }

    return b;
}

fn fib_rec(n: u32) -> u32 {
    if n <= 1 {
        return 1;
    }

    return fib_rec(n - 1) + fib_rec(n - 2);
}
