fn main() {
    let n1 = "electrical".to_string();
    let n2 = "electronic".to_string();
    let n3 = "engineering".to_string();

    let n4 = n1 + &n2 + &n3; 

    //about electrical/electronic
    println!("\nThe {} is informed by the aspiration to train electronical/electronic engineering professionals in the areas of design, building and maintenance of electrical control systems,", n4);


    let w1 = "Computer".to_string();
    let w2 = "Science".to_string();
    let w3 = w1 + &w2;
    println!();
    println!("{} is aimed at the developing competent, creative, innovative, entrepeneurial and ethical minded persons, capable of creating value in diverse fields of computer science.", w3);

}
