/*Ownership is a discipline for ensuring the safety of Rust programs. 
Rust check that variable is deined or not at the compile-time and raises an error
*/

pub fn fn_learn_ownership_concepts(){
    println!("Learn Ownersjip Concepts");
    fn_variable_memory_allocation_code();
    fn_move_variables();
    fn_heap_memory_allocation_code();
    fn_variable_clonin_code();
}

/**
 * Memory in the Stack
 * Variables Live in the Stack
 * Variables lives in frame.
 * A frame is a mapping from variables to values within a single scope, such as a function
 * Frames are organized into a stack of currently-called-functions
 * After a function scope is over, Rust deallocates the function's frame.
 * L1 -: The frame for fn_variable_memory_allocation_code n = 5
 * L2 -: The frame for plus_one holds x = 5.
 * L3 -: The frame for fn_variable_memory_allocation_code holds n = 5; y = 6.
 * 
 * Frames are organized into a stack
 */
fn fn_variable_memory_allocation_code(){
    let n = 5;      //L1 -> holds n = 5
    let y = plus_one(n); // L3 -> holds only n = 5 and y = 6

    println!("Value of n = {n}");
    println!("Value of y = {y}");
}

fn plus_one(x: i32) -> i32{
    x + 1 //L2 holds x = 5
   
}

/**
 * Heap Memory, Ownership, Move Onership and Memory deallocation
 * Box lives in heap memory
 * pointers -: is a value that describes a location in memory
 *          -: To transfer access to data without copying it, Rust uses pointers
 * pointee  -: value that a pointer points-to
 * heap     -: is a separate region of memory where data can live indefinitely. 
 *             One common way to make a pointer is to allocate memory in the heap.
 * All heap data must be owned by exactly one variable.
 * Box  -: Rust provides a construct for putting data on the heap.
 * 
 * here nameStr is in the stack. 
 * Once add_suffix is finished, Rust deallocates its stack frame. 
 * so Rust deallocates the heap data in nameStr
 * 
 * L1 -: the string "Ferris" has been allocated on the heap. It is owned by first.
 * L2 -: the function add_suffix(first) has been called. This moves ownership of the string from first to nameStr. The string data is not copied, but the pointer to the data is copied.
 * L3 -: the function nameStr.push_str(" Jr.") resizes the string's heap allocation. 
 *      This does three things. 
 *          First, it creates a new larger allocation.
 *          Second, it writes "Ferris Jr." into the new allocation.
 *          Third, it frees the original heap memory. first now points to deallocated memory.
 * L4 -: the frame for add_suffix is gone. This function returned nameStr, 
 *      transferring ownership of the string to full.
 */
fn fn_heap_memory_allocation_code(){
    let first = String::from("Ferris"); //L1
    let full = add_suffix(first); //L4
    println!("{full}");
   // println!("first value = {first}"); //cannot access the first. becasue Rust has deallocated it
}


fn add_suffix(mut name_str: String) -> String { //L2
    name_str.push_str(" Jr."); //L3
    name_str
}

/**
 * String is in the heap
 *  1. greet moves the data from m1 and m2 into the parameters of greet. 
 *  2. Both strings are dropped at the end of greet, and therefore cannot be used within main. 
 *  3. If we try to read them like in the operation format!(..), 
 *      then that would be undefined behavior.
 */
fn fn_move_variables() {
    let m1: String = String::from("Hello");
    let m2 = String::from("world");
    greet(m1, m2);
    //let s = format!("{} {}", m1, m2); // Error: m1 and m2 are moved
}

fn greet(g1: String, g2: String) {
    println!("{} {}!", g1, g2);
}

/**
 * Variable Cloning
 * .clone() method -: One way to avoid moving data is to clone it
 * 
 * 
 * L1 -: first_clone did not "shallow" copy the pointer in first
 *       but instead "deep" copied the string data into a new heap allocation. 
 * L2 -: while first_clone has been moved and invalidated by add_suffix, 
 *       the original first variable is unchanged. 
 *       It is safe to continue using first.
 */
fn fn_variable_clonin_code() {
    let first = String::from("Ferris"); //L1
    let first_clone = first.clone();  //L2
    let full = add_suffix(first_clone);
    println!("{full}, originally {first}");
}
