/*

Integer: i16, i64, i128, u8, u16, u32, u64, u128       (Default is 'i32')
                         --- Unsigned integer ---

Float: f32, f64  (Default is 'f64')

Boolean: bool

Character: char

Tuples

Arrays

*/

// code to check type of variable

pub fn print_type_of<T>(_: &T) {
    println!("Data type is {}", std::any::type_name::<T>());
}
// ------------------------------



pub fn types()
{
    let num1 = 123;
    println!("Number is: {}",num1);
    print_type_of(&num1);

    let num2 = 12.23;
    println!("\nNumber is: {}",num2);
    print_type_of(&num2);

    // explicitly defining type of variable

    let a: i64 = 32434;
    println!("\nNumber is: {}",a);
    print_type_of(&a);

    let b: f32 = 342.342;
    println!("\nNumber is: {}",b);
    print_type_of(&b);

    // -----------------------------------

    let is_active = true;
    println!("\nValue is: {}", is_active);
    print_type_of(&is_active);


    // -----------------------------------

    let is_greater: bool = 12 < 5;
    println!("\nValue is: {}", is_greater);
    print_type_of(&is_greater);





}


