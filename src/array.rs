// Length of array is compile time constant and all values are of same type

pub fn array()
{
    /* Syntax
          let name_of_array: [array_type ; size_of_array] = [elements]

    */
    let arr: [i32 ; 5] = [12,23,234,12,2343];     // immutable array

    println!("Elements in arr is: {:?}",arr);

    // creating mutable array

    let mut arr1: [i32 ; 7] = [1,2,3,4,5,6,7,];

    println!("Element in arr1 is: {:?}",arr1);

    arr1[5] = -32;

    println!("Element in arr1 is: {:?}",arr1);

   // printing a single element

   println!("Element at index 3 is: {}",arr1[3]);

   // printing length of array

   println!("Length of arr1 is: {}",arr1.len());

    // slicing
   println!("Element in arr1 from index 0 to 3 is: {:?}", &arr1[0..3]);

   println!("Element in arr1 from index 0 to n is: {:?}", &arr1[0..]);  // n represents last element

   println!("Element in arr1 from index start to index 4 is: {:?}", &arr1[..4]);

   println!("Element in arr1 from start to end is: {:?}", &arr1[..]);

}