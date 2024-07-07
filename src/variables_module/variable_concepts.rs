use std::char;

/**
 * 1. By default, variables are immutable.
 * 2. const: used to make a variable constant and cannot use mut for constant variables
 *      Constants are valid for the entire time a program runs, within the scope in which they were declared
 * 3. shadowing is declaring a new variable with the same name as a previous variable

 */
const CONST_VALUE_GLOBLE:i32  = 6; //globally defined
pub fn fn_learn_variables_concepts(){
    fn_demo_variables();
    fn_demo_constants();
    fn_demo_shadowing();
    fn_demo_compound_tuple_data_type();
    fn_demo_compound_array_data_type();
}

fn fn_demo_variables(){
    let x = 5;
    println!("Variable Value {x}");
}

/*2. Constant variables */
fn fn_demo_constants(){
    println!("\n******Demo Constants*********");
    const CONST_VALUE_LOCAL:i32  = 1 + 1; //defined in the function
    println!("Constant Value {CONST_VALUE_GLOBLE}");
    println!("Constant Value {CONST_VALUE_LOCAL}");
}

/*3. Shadowing Variables */
fn fn_demo_shadowing(){
    println!("\n******Demo Shadowing*********");
    let x = 5;
    let x = x + 2;

    {
        let x = x * 2;
        println!("Value in inner scope: {x}"); // 14
    }
    println!("Value in outer scope: {x}");// 7


    //-------------------------------
    println!("\n******Demo Shadowing -  Type Change *********");
    let spaces = "   ";
    let spaces = spaces.len();
    println!("Value of spaces: {spaces}");// length of spaces string
}


/**
 * Compound types -: can group multiple values into one type.
 * Rust has two primitive compound types: 
 *          > tuples
 *          > arrays.
 * 
 * > Tuples have a fixed length: once declared, they cannot grow or shrink in size.
 */
fn fn_demo_compound_tuple_data_type(){
    println!("\n******Demo Data Type -  Compound Type - Tuple *********");
    let tup : (i32, f64, u8, char) = (500, 6.4, 1, 'K');
    println!("{:?}", tup);

    println!("\n******Demo Data Type -  Compound Type - Get Individual Elements *********");
    //destructuring -> breaks the single tuple into separate parts
    let(x, y, z, c) = tup;
    println!("Value x = {}", x);
    println!("Value y = {}", y);
    println!("Value z = {}", z);
    println!("Value c = {}", c);


    println!("\n");
    //destructuring -> breaks the single tuple into separate parts using period(.)
    let v_x = tup.0;
    let v_y = tup.1;
    let v_z = tup.2;
    let v_c = tup.3;
    println!("Value v_x = {}", v_x);
    println!("Value v_y = {}", v_y);
    println!("Value v_z = {}", v_z);
    println!("Value v_c = {}", v_c);

}

/**
 * Array is used to keep collection of elements with same type and fix size
 * data keep in stack not in heap
 */
fn fn_demo_compound_array_data_type(){
    println!("\n******Demo Data Type -  Compound Type - Array *********");
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];

    println!("First element of the array : {}", first);
    println!("Second element of the array : {}", second);
}
