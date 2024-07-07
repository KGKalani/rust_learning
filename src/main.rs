//#[path = "game_module/gussing_game.rs"] mod gussing_game; // can import separate module like this
mod game_module;
mod variables_module;
mod function_module;

fn main() {
    println!("Hello, Rust Learners!");

   /*gussing game*/
   // game_module::gussing_game::fn_gussing_game();

    /*Variable Concepts */
    variables_module::variable_concepts::fn_learn_variables_concepts();

    /*Function Concepts */
    function_module::function_concepts::fn_learn_function_concepts();

}

