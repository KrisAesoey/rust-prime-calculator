fn is_prime(x: i32) -> bool {
    let x_sqrt : f64 = x.into();
    let x_sqrt : i32 = x_sqrt.sqrt() as i32;
    if x < 2 {return false}
    else if x == 2 {return true}
    else if x % 2 == 0 {return false}
    for i in (3..(x_sqrt)).step_by(2) {
        if x % i == 0 {return false}
    }
    true
}

fn factorize_primes(mut n: i32) -> Vec<i32> {
    let mut factors = Vec::new();

    while n % 2 == 0 {
        factors.push(2);
        n = n / 2;
    }

    let n_sqrt : i32 = (n as f64).sqrt() as i32;

    for i in (3..(n_sqrt)).step_by(2) {
        while n % i == 0 {
            factors.push(i);
            n = n / i;
        }
    }
    if n > 2 {
        factors.push(n);
    }
    factors
}

fn main() {
    println!("Welcome to prime calculator!");
    println!("Please write a number:");
    
    let mut number = String::new(); //defined mutable variable

    std::io::stdin()
        .read_line(&mut number) //& reference to var that is mutable
        .expect("Failed to read line");

    let number: i32 = number.trim().parse().expect("Please type a number!");

    println!("You wrote: {}", number);

    println!("What do you wish to do with your number?");
    println!("[1] Check if it's prime");
    println!("[2] Find prime factorization");

    let mut choice = String::new();
    
    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read line");

    let choice : i32 = choice.trim().parse().expect("Please type a number!");

    if choice % 2 == 0 {
        let factors = factorize_primes(number);

        for e in factors {
            println!("{}", e)
        }
    }

    else {
        if is_prime(number) {
            println!("Your number is a prime number!")
        }
        else {
            println!("Your number is not a prime number...")
        }
    }
}