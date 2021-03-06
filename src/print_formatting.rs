// public function 
pub fn message()
{
    println!("Inside print_formatting.rs file");

    // printing number 
    println!("{}",1);
    println!("Number: {} and {}", 1,7);

    // string formatting
    println!("Age of {} is: {}", "Bob",20);

    // positional arguments
    println!("{0} is from {1} and {0} like {2}", "Ali","Pakistan","code");

    // Named Argument
    println!("{name} like to play {activity}",
     name = "Hamza",
    activity = "Backetball");

    // Placeholder traits
    println!("Binary: {:b}, Hexadecimal: {:x}, Octal: {:o}", 10,10,10);
    println!("In binary {0} is: {:b}\nIn Hexadecimal {1} is {:x}\nIn octal {2} is: {:o}",20,20,20);

    // Placeholder for debug trait
    println!("{:?}",(15,true,"Hello"));

    // Basic Math
    println!("10 + 10 is: {}", 10+10);
    println!("{} + {} is: {}", 20,10,20+10);
    println!("After multiplying {num1} and {num2}, Result is: {result}",num1 = 10, num2 = 5, result = 10 * 5);

}