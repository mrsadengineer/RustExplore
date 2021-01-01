pub fn example_prim(){
    let a = 1;

    let b = 2.0;
    let name = "My Name is Rust";
    let boolean = true;
    println!("The value of {} is:", a);
    println!("The value of {} is:", b);
    println!("The value of {} is:", name);
    println!("The value of {} is:", boolean);

    //arrays
    let array1 : [i16; 5] = [1,2,4,5,8];
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    //print 20

    println!("The value of a is: {}", arr[1]);

    //Tuples
    let tuple1 = (5, 6.0, "My_Name");
    println!("Hello, world!");
    println!("Array i16 - {}", array1[2]);
    // let arithmetic_value= 8-2 +8;
    // println!(arithmetic_value);
    hello("Rust");

    println!("Rust -{}", tuple1.1);

   
    add(2, 3);
}


fn hello(name: &str) {
    println!("hello {}", name);
}

fn add(a: i8, b: i8) {
    println!("{}", a + b);
}