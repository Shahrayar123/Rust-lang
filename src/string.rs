pub fn string()
{ 
    let mut msg = String::from("Hello ");

    println!("Message is: {}",msg);

    // print length of string
    println!("Length of world {} is {}", msg, msg.len());

    // Pushing character in existing word
    msg.push('W');

    println!("\nMessage is: {}",msg);

    // Pushing string in existing word
    msg.push_str("orld");

    println!("\nMessage is: {}",msg);

    // Checking capacity of string in bytes
    println!("\nCapacity is: {}", msg.capacity());
    
    // Checking whether the string is empty or not
    println!("\nIs Empty: {}", msg.is_empty());

    // Checking whether the string contain specific word(s) or not
    println!("\nContain 'world': {}", msg.contains("world"));
    
    println!("\nContain 'World': {}", msg.contains("World"));

    // Replacing word
    
    println!("\nExisting word is: {}", msg);

    println!("\nAfter replacing, the word is: {}", msg.replace("World", "there"));
    


}