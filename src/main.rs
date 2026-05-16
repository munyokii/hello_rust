// VARIABLES, CONSTANTS, SHADOWING
fn variables_const_shadowing() {
   const MAX_VALUE:i32 = 50;
   println!("{}", MAX_VALUE);

   let mut x:i64 = 5;
   println!("Display value of x: {}.", x);

   x = 6;

   println!("Display value of x: {}.", x);
}

// DATA TYPES
fn data_types() {
   let age: i32 = 20;
   let price: f64 = 99.5;
   let is_student: bool = true;
   let grade: char = 'A';
   let name: &str = "Shiva";

   println!("Name: {}"
   , name);
   println!("Age: {}"
   , age);
   println!("Price: {}"
   , price);
   println!("Student: {}"
   , is_student);
   println!("Grade: {}"
   , grade);
}

// FUNCTIONS WITH PARAMETERS
fn greet(name:&str) {
   println!("Halo halo {}.", name);
}

fn add(a:i32, b:i32) -> i32 {
   a + b
}


fn main() {
   variables_const_shadowing();
   data_types();
   greet("muli");

   let result = add(32, 65);
   println!("Result is: {}", result);
}