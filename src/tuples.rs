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

    let a = (1,2,3,4,5,6,7,8,9,10);

    let b = (a, 11,12,13,14,15);

    println!("\n{:?}",b);

    println!("\n{:#?}",b.0);


}