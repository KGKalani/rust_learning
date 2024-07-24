

pub fn fn_learn_control_flow_concepts(){
    println!("\n<<<<<<<<<<<<<<<<<Let's Learn Control Flow Concepts>>>>>>>>>>>>>>>>>");
    println!("==============================================");
    println!("\n*****Conditional Statements*********");
    fn_demo_if_else_statement();
    fn_demo_if_elseif_else_statement();
    fn_demo_using_if_in_let_statement();

    println!("\n*****Loops*********");
    fn_demo_loop();
    fn_demo_loop_with_return_value();
    fn_demo_loop_label();
    fn_demo_while_loop();
    fn_demo_for_loop()
}


fn fn_demo_if_else_statement(){
    println!("\n******Demo if - else Statement*********");
    let number = 6;

    if number > 5 {
        println!("Condition is true. Number {number} is grater than 5.");
    }
    else{
        println!("Condition is false. Number {number} is less than or equal to 5");
    }
}

fn fn_demo_if_elseif_else_statement(){
    println!("\n******Demo if - else if - else Statement*********");
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
}


fn fn_demo_using_if_in_let_statement(){
    println!("\n******Demo using if in let Statement*********");
    let number = 6;
    let number = if number > 5 { number } else { 0 };

    println!("Condition true. The value of number is: {number}");

    let number = 2;
    let number = if number > 5 { number } else { 0 };

    println!("Condition false. The value of number is: {number}");
}

/**
 * The loop keyword tells Rust to execute a block of code over and over again forever 
 * or until you explicitly tell it to stop.
 * Terminals support the keyboard shortcut ctrl-c to interrupt a program
 */
fn fn_demo_loop(){
    println!("\n******Demo for loop*********");
    let mut count = 0;
    loop {
        println!("again!");
        count += 1;

        if count == 10 {
            println!("count is {count}");
            break;
        }
    }
}

fn fn_demo_loop_with_return_value(){
    println!("\n******Demo for loop with return value*********");
    let mut count = 0;

    let result = loop {
        count += 1;

        if count == 10 {
            break count * 2; //here adding semicolon is optional
        }
    };

    println!("Result is {result}");
}

/**
 * Loop label can be set for inner and outer loop to avoid the disambiguation.  
 * This label can be used with break and continue key word to specify to which loop we assign this key word.
 * Loop label must begin with a single quote
 */
fn fn_demo_loop_label(){
    println!("\n******Demo for loop -: Loop Label*********");
    let mut count = 0;

    'counting_up: loop{              // set a loop label to outer loop
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {    // this will break the inner loop
                break;
            }
            if count == 2 {        // this will break the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}


fn fn_demo_while_loop(){
    println!("\n******Demo for loop -: while loop*********");
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        number -= 1;
    }
    println!("LIFTOFF!!!");
}

/**
 * for loop can be used to iterate the elements in an array
 */
fn fn_demo_for_loop(){
    println!("\n******Demo for loop -: for loop *********");
    let a = [10, 20, 30, 40, 50];

    println!("Elements in the array [10, 20, 30, 40, 50]");
    for element in a { //loop the elements in 
        println!("the value is: {element}");
    }

    println!("\nNumbers in range 1..4");
    for number in (1..4).rev() {
        println!("Numbers : {number}!");
    }
    println!("LIFTOFF!!!");
}