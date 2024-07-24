use std::io;

use assignement_module::common_concepts_examples;
use common_concepts_module::variable_concepts;
use common_concepts_module::function_concepts;
use common_concepts_module::control_flow_concepts;
use ownership_module::fixing_ownershi_errors;
use ownership_module::ownership_concept;
use ownership_module::reference_and_borrowing_concepts;
use ownership_module::slice_type;


//#[path = "game_module/gussing_game.rs"] mod gussing_game; // can import separate module like this
mod game_module;
mod assignement_module;
mod common_concepts_module;
mod ownership_module;



fn main() {
    println!("Hello, Rust Learners!");    
    
    loop{
        println!("\n********** Which Module do you want to execute?\nEnter the Module Code ********");
        println!("Code  Name");
        println!("0.    Gussing Game");
        println!("1.    Variable Concepts");
        println!("2.    Function Concepts");
        println!("3.    Control Flow Concepts");
        println!("4.    Ownership Concepts");
        println!("99.   Assignment - Common Concepts");
        println!("100   Exit");

        let mut module_code = String::new();

        io::stdin()
        .read_line(&mut module_code) // get user input
        .expect("Failed to read line");

        let module_code: u32 = match module_code.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter a Number");
                continue;
            }
        };


        if module_code == 0 {
            /*gussing game*/
            game_module::gussing_game::fn_gussing_game();
        } 
        else if module_code == 1 {
            /*Variable Concepts */
            variable_concepts::fn_learn_variables_concepts();
        } 
        else if module_code == 2 {        
            /*Function Concepts */
            function_concepts::fn_learn_function_concepts();
        } 
        else if module_code == 3 {        
            /*Function Concepts */
            control_flow_concepts::fn_learn_control_flow_concepts();
        }
        else if module_code == 4 {        
            /*Ownership Concepts */
            ownership_concept::fn_learn_ownership_concepts();
            reference_and_borrowing_concepts::fn_learn_reference_and_borrowing_concepts();
            fixing_ownershi_errors::fn_learn_fixing_ownership_errors();
            slice_type::fn_learn_slice_type();
        }
        else if module_code == 99 {
            common_concepts_examples::demo_common_concept_assignments();
        }
        else if module_code == 100 {        
            /*Function Concepts */
            println!("Exit from RUST Learning");
            break;
        }
        else {
            println!("No such code has been defined")
        }
    }

    


}

