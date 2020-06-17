fn main() {
    // let mut x = 10;
    // let x2 = &x;
    // let x3 = &mut x;

    // println!("x is {}", x);
    // println!("x is {}", x2);
    // println!("x is {}", x3);

    // *x3 += 1;

    // println!("x is {}", x);
    // println!("x is {}", x2);
    // println!("x is {}", x3);

    let mut x = 0;

    let x2 = &mut x;
    *x2 += 1;

    println!("{}", x);
}
