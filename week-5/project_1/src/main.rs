fn main() {
  let a:i32=2;//coefficient of x^2
  let b:i32=6;//coefficient of x
  let c:i32=-8;//constant

  let result:i32;

  result = -b+(b^2 - 4&(a&c))/2&a;
  println!("(-b+(b^2 - 4(a&c))/2a)=>{}",result);

}
