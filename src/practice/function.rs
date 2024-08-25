pub fn without_param() {
    println!("Without param");
}

pub fn single_param(a: i32) {
    println!("value of a is: {}", a);
}

pub fn two_param(a:i32, b:char) {
    println!("value of a and b are: {}, {}", a, b);
}

pub fn sum(p0: i32, p1: i32) {
    println!("Sum of p0 and p1 is: {}", sum_internal(p0, p1));
}

fn sum_internal(p0: i32, p1: i32) -> i32 {
    p0 + p1
}