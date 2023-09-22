fn next_val(a: &mut i32, b: &mut i32) {
    let c = *a + *b;
    *a = *b;
    *b = c;
}

fn fib_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;

    println!("====> loop start ...");

    loop {
        if i >= n {
            break;
        }

        //let c = a + b;
        //a = b;
        //b = c;
        next_val(&mut a, &mut b);

        println!("next val is {}", b);

        i += 1;
    }

    println!("====> loop end ...");
}

fn fib_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    println!("====> while start ...");

    while i < n {
        //let c = a + b;
        //a = b;
        //b = c;
        next_val(&mut a, &mut b);

        println!("next val is {}", b);

        i += 1;
    }

    println!("====> while end ...");
}

fn fib_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    println!("====> for start ...");

    for _i in 2..n {
        //let c = a + b;
        //a = b;
        //b = c;
        next_val(&mut a, &mut b);

        println!("next val is {}", b);
    }

    println!("====> for end ...");
}

fn main() {
    let n = 10;

    fib_loop(n);

    fib_while(n);

    fib_for(n);
}

