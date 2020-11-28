use std::io::{stdin, stdout, Write};

fn input(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}

fn main() {
   let mut num1 = String::new();

   let mut num2 = String::new();

   let mut operation = String::new();

   print!("Please input the first number: ");
   input(&mut num1);

   print!("Please input the second numner: ");
   input(&mut num2);

   print!("Please input the operation: ");
   input(&mut operation);

  let num1 = num1.trim().parse::<i32>().unwrap();
  let num2 = num2.trim().parse::<i32>().unwrap();
  let operation = operation.trim().parse::<char>().unwrap();
 
  let result = match operation {
    '+' => num1 + num2,
    '-' => num1 - num2,
    '*' => num1 * num2,
    '/' => num1 / num2,
    _ => panic!("unknown operation")
  };

  println!("{} {} {} = {}", num1, operation, num2, result);
}
