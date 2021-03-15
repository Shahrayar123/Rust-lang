// Infinite loop -----> loop {}
// while loop  -------> while {}
// for range loop ------> 



pub fn run()
{
    let mut num: i32 = 1;

    // Infinite loop

    // loop
    // {
    //     println!("Number is: {}", num);

    //     if num == 30
    //     {
    //         break;
    //     }

    //     num += 1;

    // }

    // while loop

    println!("\n");

    // while num <= 50
    // {
    //     if num % 2 == 0
    //     {
    //         println!("Number {} is even", num);
    //     }

    //     else
    //     {
    //         println!("Number {} is odd", num);
    //     }

    //     num += 1;
    // }

    println!("\n");

    // for loop

    for i in 1..50
    {
        if i == 20
        {
            continue;
        }
        
        else if i == 30
         {break}

        else
        {      
            if i % 2 == 0
            {
                println!("Number {} is even", i);
            }

            else
            {
                println!("Number {} is odd", i);
            }

        }

    }



}