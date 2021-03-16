pub fn run()
{
    let sum;
    // function call of adding 3 numbers
    add(3,4,6);
    
    // function call of sum even numbers in specified range and store the returned result in 'sum'
    sum = sum_even(1,50);

    println!("\nSum of even numbers in specified range is: {}", sum);
}


// function that print sum of 3 numbers
fn add(n1: i32 , n2: i32 , n3: i32)
{
    let temp_var = n1 + n2 + n3;

    println!("Sum of {}, {} and {} is: {}", n1,n2,n3, temp_var);

}

// function that return sum of even numbers in range

fn sum_even(low: i32, high: i32) -> i32
{
    let mut sum: i32 = 0;

    for num in low..high
    {
        if num % 2 == 0
        {
            sum += num;
        }    
    }

    return sum;

}

