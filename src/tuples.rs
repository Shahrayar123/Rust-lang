// Tuple group togather values of different types


pub fn tuple()
{
    let items = ("Apple","Orange",123, true, false, 0.232,434.23,'S');

    println!("First element in tuple is: {}",items.0);
    println!("Second element in tuple is: {}",items.1);
    println!("Third element in tuple is: {}",items.2);
    println!("Fourth element in tuple is: {}",items.3);
    println!("Fifth element in tuple is: {}",items.4);
    println!("Sixth element in tuple is: {}",items.5);
    println!("Seventh element in tuple is: {}",items.6);

 
    // assigning values from tuple to variables
    let  (a,b,c,d,e,f,g,h) = items; 



}