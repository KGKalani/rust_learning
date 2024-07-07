/**
 * function is a series of statements optionally ending in an expression (return value)
 *      > Can have parameters -> fn get_sum(a: u32, b: u32)
 *      > Can have return type ->  fn get_sum(a: u32, b: u32) -> u32 {}
 * 
 * Statements:  are instructions that perform some action and do not return a value.
 * Expressions:  evaluate to a resultant value.
 */

 pub fn fn_learn_function_concepts(){
    fn_without_parameters_and_return_type();
    
    let z = fn_with_parameters_and_return(5);
    println!("Value of x + y = {z}");

    fn_only_with_statements(6);
 }

 fn fn_without_parameters_and_return_type(){
    println!("\n******Demo Function without Parameters and Return Types*********");
    let x = 3; // this is a statement.
    println!("Value of x : {x}")
 }

 /**
  * This function contains 
    > statement : let x = 3
    > expression: x + y
    > return type : x + y

    if we add a semicolon at the end of x + y, it truns to a statement
  */
 fn fn_with_parameters_and_return(y: i32) -> i32{
    println!("\n******Demo Function with Parameters and Return Types*********");
    let x = 3; // this is a statement.
    return x + y  // no semicolon at the end. This is an expression and returns sum of x and y
 }

 /**
  * function only contains statements
  */
 fn fn_only_with_statements(y: i32){
    println!("\n******Demo Function with statements. No return types*********");
    let x = 3; // this is a statement.
    let z = {
        let x = 3;
        x + 1
    };
    x + y + z;  // This is a statement because there is a semmicolon at the end of x + y, this is not return
 }

