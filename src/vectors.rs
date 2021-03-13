// Vectors are resizable arrays

pub fn vector()
{
    let nums: Vec<i32> = vec! [12,32,234,13,23,2,31,434,3132];   // immutable vector

    // Displaying elements of vectors

    println!("Number at index 3 is: {}",nums[3]);

   // nums.push(2);    // cannot append number because 'nums' is immutable

   let numbers: Vec<i32> = Vec::new();    // 1 way to make vector


   let mut nums: Vec<i32> = vec! [12,32,234,13,23,2,31,434,3132];   // immutable vector

   println!("Vector is: {:?}", nums);

    //   displaying length of vector

   println!("\nLength of vector is: {}", nums.len());

   // push numbers in vector
    nums.push(12);
    nums.push(13);
    nums.push(14);
    nums.push(15);

    println!("\nAfter pushing number vector is: {:?}", nums);

    // pop numbers

    nums.pop();
    nums.pop();
    nums.pop();

    println!("\nLength of vector is: {}", nums.len());

    println!("\nAfter pop vector is: {:?}", nums);

    // displaying length of vector

    println!("\nLength of vector is: {}", nums.len());

}