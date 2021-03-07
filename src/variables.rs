// In Rust variables are immutable by default
// To make variable mutable use 'mut' keyword before variable

pub fn understand_variables()
{
    println!("Inside variables.rs file");

    // declearning variable
    let num1 = 123;

    println!("\nNumber is: {}",num1);
    
    // num1 = 123334;    / cannot do this as by default variables in Rust is immutable
    
    // Make mutable variable

    let mut age = 16;
    let name = "Khalid";

    println!("\nAge of {} is {}",name, age);

    // overighting age variable
    age = 17;

    println!("Age of {} is {}",name, age);

    // -------------------------------------------

    let mut flag = false;
    
    println!("\nFlag value is: {}", flag);

    flag = true;

    println!("Flag value is: {}", flag);

    // defining constants
    const ID: i32 = 005;

    println!("\nID is :{}",ID);

    // -------------------------------------------

    // Assigning multiple values in multiple variables

    let (name , age , class, address) = ("Ali",16, 7, "ISB");

    println!("\nName of student is: {}\nAge of student is: {}\nClass of student is: {}\nAddress of student is: {}", name, age, class, address);

}