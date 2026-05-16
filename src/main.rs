// -------------UNCOMMENT TO TEST-----------


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


// CONTROL FLOW - Control flow decides how a program runs and which code executes
// if STATEMENT - if is used to make decisions based on a condition
// else STATEMENT - else runs when the if condition is false.
// else if STATEMENT - else if checks another condition if previous condition is false.


fn if_statement() {
   let marks = 75;
   if marks >= 90 {
      println!("Grade A");
   } else if marks >= 70 {
      println!("Grade B");
   } else if marks >= 35 {
      println!("Pass");
   } else {
      println!("Fail");
   }
}


// LOOPS - Loops are used to repeat code multiple times
// FOR LOOPS - for loop repeats code for a fixed range or collection

fn for_loop() {
   for i in 1..= 5 {
      println!("Round {}", i)
   }
}

// WHILE LOOPS - while loop runs as long as a condition is true

fn while_loop() {
    let mut count = 1;

    while count <= 5 {
      println!("{}", count);

      count += 1;
    }
}

// LOOP - loop creates an infinite loop.

fn loops_loop() {
    loop{
      println!("Running...")
    }
}

// PROGRAM WITH BREAK

fn program_with_break() {
    let mut count = 1;

    loop {
      println!("{}", count);

      if count == 5 {
         break;
      }

      count += 1;
    }
}


fn main() {
   variables_const_shadowing();
   data_types();
   greet("muli");

   let result = add(32, 65);
   println!("Result is: {}", result);

   if_statement();
   for_loop();
   while_loop();
   loops_loop();
   program_with_break();
}