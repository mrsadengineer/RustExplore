fn array_ex1(){
    let array1 : [i16; 5] = [1,2,4,5,8];
    println!("Array i16 - {}", array1[2]);
}

pub fn array_exAll(){
array_ex1();
array_02();
}

fn array_02(){
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of a is: {}", arr[1]);

}