use std::mem;

pub fn all_data_types() {

    //declaration
    let i: i32;

    //assigning value
    i=5;
    println!("{}", i);

    let tup_default = (1,2,5);
    let tup_mix:(i32,u32,f32) = (1,2,3.0);
    println!("Default tuple is: {:?}", tup_default);
    println!("Mix tuple is {:?}", tup_mix);

    //fetch values from tuple
    println!("Second value of tup_default is: {}", tup_default.1);

    //tuple declaration
    let tup_dec: (i32,i32,i32);
    tup_dec = (1,2,3);

    // array

    let array_assigned = [1,2,3,4,5];

    println!("Second value of array_assigned: {}", array_assigned[1]);

    // Fixed-size array (type signature is superfluous).
    let fixed_size_array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Fixed size array is: {:?}", fixed_size_array);

    // All elements can be initialized to the same value.
    let ys: [i32; 3] = [0; 3];
    println!("All elements initialized with same value: {:?}", ys);

    println!("Length of fixed_size_array: {}", fixed_size_array.len());

    // Arrays are stack allocated.
    println!("Array occupies {} bytes", mem::size_of_val(&fixed_size_array));

    //Slice

    let slice1 = &fixed_size_array;
    let slice2 = &fixed_size_array[1..2];

    println!("slice1 is: {:?}", slice1);
    println!("slice2 is: {:?}", slice2);
}