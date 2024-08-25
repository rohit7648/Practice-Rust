pub fn ownership_1() {

    let x = String::from("hello, world 1");
    let mut y = x.clone();
    println!("{},{}",x,y);
}

pub fn ownership_2() {

    let x = "hello, world 2";
    let y = x;
    println!("{},{}",x,y);
}

pub fn ownership_3() {

    let x = &String::from("hello, world 3");
    let y = x;
    println!("{},{}",x,y);
}

pub fn ownership_4() {
    let x = String::from("hello, world 4");
    let y = x.as_str();
    println!("{},{}",x,y);
    ownership_5();
}

fn ownership_5() {
    let mut x = String::from("hello, world 5"); // `x` must be mutable
    let y = &mut x; // `y` is a mutable reference to `x`

    // Modifying the data through `y`
    y.push_str("!!!");

    // println!("x: {}", x); // uncommenting this will throw error because y is still in scope and have mutable reference to x
    println!("y: {}", y); // `y` also reflects the modified data
    ownership_6();
}

fn ownership_6() {
    let mut x = String::from("hello, world 6"); // `x` must be mutable
    let y = &mut x; // `y` is a mutable reference to `x`

    y.push_str("!!!");
    // Modifying the data through `y`

    println!("x: {}", x); // y is in scope but still it works but it's not a good way of coding according to rust
    ownership_7();
}

fn ownership_7() {
    let mut x = String::from("hello, world 7"); // `x` must be mutable

    {
        let y = &mut x; // `y` is a mutable reference to `x`
        y.push_str("!!!");
        println!("y: {}", y);
    }

    println!("x: {}", x); // scope of y is gone now you can use x
    ownership_8();
}

fn ownership_8() {

    let x = String::from("hello, world 8"); // `x` must be mutable

    {
        let mut y =  x; // `y` is a mutable reference to `x`
        y.push_str("!!!");
        println!("y: {}", y);
    }

    //println!("x: {}", x); // uncomment this and check scope of x is gone this will throw error
    ownership_9()
}

fn ownership_9() {
    let s1 = String::from("Hello world 9");
    let s2 = take_ownership(s1);
    println!("{}", s2);
}

fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}

fn ownership_10() {
    let s = give_ownership_1();
    println!("{}", s);
}

fn give_ownership_1() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.into_bytes();  // .into_byes() transfers the ownership of s and modifies the value into vector
    // s // uncommenting this  will throw error since ownership is transferred
    give_ownership_2()
}

fn give_ownership_2() -> String {
    let s = String::from("Hello world");
    // Convert String to Vec
    let _s = s.as_bytes();  // here s is borrowed as reference not owned the value
    s
}