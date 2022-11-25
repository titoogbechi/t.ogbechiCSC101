// The iter() function fetches values of all elements in array.

fn main() {
    let arr:[&str;3] = ["10","20","30"];
    println!("array is {:?}",arr);
    println!("array size is :{}",arr.len());
 
 
 for val in arr.iter(){
    println!("Value is :{}",val);
 }
 }
 