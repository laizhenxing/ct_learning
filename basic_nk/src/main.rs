fn main() {
    println!("loop: {:?}", fbi_loop(7));
    println!("for: {:?}", fbi_for(7));
    println!("while: {:?}", fbi_while(7));
}

fn fbi_loop(n: u8) {
    let mut a = 1;
    let mut b = 1;
    let mut i: u8 = 2;

    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        println!("next number is: {}", b);

        if i >= n {
            break;
        }
    }
}

fn fbi_for(n: u8) {
    let (mut a, mut b) = (1, 1);

    for _i in 2..n {
        let c = a + b;
        a = b;
        b = c;
        println!("next number is: {}", b);
    }
}

fn fbi_while(n: u8) {
    let (mut a, mut b, mut i) = (1, 1, 2);

    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
        println!("next number is: {}", b);
    }
}

// fn fbi_next(a:&mut i32, b:&mut i32) -> i32 {
//     let mut c = *a + *b;
//     a = b;
//     b = c;
//     c
// }