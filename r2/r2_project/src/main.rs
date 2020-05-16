static PI: f32 = 3.14;

fn sum(a: i64, b: i64) -> i64 {
    return a + b;
}

fn main() {
    let mut n: u8 = 8; // chỉ có thể khai báo trong fn
    println!("PI = {}", PI);
    println!("n = {}", n);
    n = 9;
    println!("n = {}", n);

    let str: String = "asudhad".to_string();
    println!("{}", str);

    println!("SUM a + b = {}", sum(123, 254));
}
