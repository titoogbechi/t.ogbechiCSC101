fn main() {
    
   let fullname = "Titoluwanimi Samuel Ogbechi";
   let department = "Computer Science";
   let uni = "Pan Atlantic university";

   let mut school = "School of science".to_string();
   //push string
   school.push_str(" and Technology");

   println!("My name is {}", fullname);
   //check length
   println!("The length my fullname is: {}", fullname.len());
   println!("I am a student of {} department", department);
   println!("{}", school);
   println!("{}", uni);
}
