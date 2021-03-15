// && ---> AND operator
// || ---> OR operator


// use std::io;

pub fn condition()
{
    let age = 21;

    // let mut name = String::new();

    // match io::stdin().read_line(&mut name)

    
    if age >= 18
    {
        println!("Your age is {} so you are eligible to vote", age);
    }

    else
    {
        println!("You are not eligible to vote");
    }

    // Shorthand if/else

    let is_age_greater = if age >= 18 {true} else {false};

    println!("\n{}", is_age_greater);

    
    
}