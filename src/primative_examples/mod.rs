mod primativeexamples;
mod arrays_ex;

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
    arrays_ex::array_exAll();


    //Tuples
    let tuple1 = (5, 6.0, "My_Name");
    
    println!("Rust -{}", tuple1.1);
    
    
    //literals and operations
    //let arithmetic_value = 8-2 +8;
    //println!(arithmetic_value);
    primativeexamples::hello("jefe");
    primativeexamples::add(2, 3);
}
