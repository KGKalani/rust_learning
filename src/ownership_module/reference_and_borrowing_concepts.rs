/**
 * A reference is a kind of pointer.
 * 
 */
pub fn fn_learn_reference_and_borrowing_concepts(){
    println!("Learn Reference and Borrowing Concepts");
    fn_reference_code();
    fn_immutable_borrowing();
    fn_mutable_borrowing();
    fn_dereferecing_a_pointer_access();
    fn_simultaneous_aliasing_and_mutation();
}

/**
 * 1. Reference and Borrowing
 * --------------------------------------------
 * expression &m1 -: uses the ampersand operator to create a reference to (or "borrow") m1.
 * The type of the greet parameter g1 is changed to &String, meaning "a reference to a String".
 * 
 * L2 -: that there are two steps from g1 to the string "Hello". 
 *      g1 is a reference that points to m1 on the stack, 
 *      m1 is a String containing a box that points to "Hello" on the heap.
 *      m1 owns the heap data "Hello", 
 *      g1 does not own either m1 or "Hello". 
 *      Therefore after greet ends and the program reaches 
 * L3, no heap data has been deallocated. 
 *      Only the stack frame for greet disappears. 
 *      This fact is consistent with our Box Deallocation Principle. 
 *      Because g1 did not own "Hello", Rust did not deallocate "Hello" on behalf of g1.
 * 
 * References are non-owning pointers, because they do not own the data they point to.
 * 
 */
fn fn_reference_code(){
    println!("\n******** Demo of Reference and Borrowing ********");
    let m1 = String::from("Hello");
    let m2 = String::from("world"); // L2
    greet(&m1, &m2); // note the ampersands
    let s = format!("{} {}", m1, m2); //L3
    println!("Value s = {s}");
}

fn greet(g1: &String, g2: &String) { // note the ampersands
    println!("{} {}!", g1, g2); //L2
}

/**
 * 2. Borrowing : borrowing is a way to access data without taking ownership of it.
 * 
 *  > Immutable Borrowing (&T):
        Allows you to read the value but not modify it. 
        Multiple immutable references to the same value can coexist.
 *  > Mutable Borrowing (&mut T):
        Allows you to modify the value.
        Only one mutable reference to a particular piece of data is allowed at any time.
        Cannot coexist with immutable references to the same value.
 */
fn fn_immutable_borrowing() {
    println!("\n******** Demo of Immutable Borrowing ********");
    let x = 5;       // x owns the value 5
    let y = &x;      // y borrows x immutably

    println!("x: {}, y: {}", x, y); // Both x and y can be used
}

fn fn_mutable_borrowing() {
    println!("\n******** Demo of Mutable Borrowing ********");
    let mut x = 5;       // x owns the value 5 and is mutable
    let y = &mut x;   // y borrows x mutably

    *y += 1;                    // Modify the value through y
    println!("y: {}", y);       // x cannot be used here, only y
}
/*
 * 2. Dereferencing a Pointer accesses its Data
 * '*' -> dereference operator
*/
fn fn_dereferecing_a_pointer_access(){
    println!("\n******** Demo of Dereferencing a Pointer accesses its Data ********");
    let mut x: Box<i32> = Box::new(1); //allocate an integer on the heap and store it in x
    println!("let mut x: Box<i32> = Box::new(1);");
    println!("Value of x -: {} ", x);
    println!("Value of *x -: {} ", *x); // Dereference x to get the value and print it // here *x is same as x


    let a: i32 = *x;         // *x reads the heap value, so a = 1
    println!("\n let a = *x");
    println!(" Value of a = {a}");

    *x += 1;                 // *x on the left-side modifies the heap value,
                             // so x points to the value 2
    
    let r1: &Box<i32> = &x;  // r1 points to x on the stack
    let b: i32 = **r1;       // two dereferences get us to the heap value
    println!("\n let r1: &Box<i32> = &x;");
    println!(" let b: i32 = **r1; ");
    println!(" Value of b (b = **r1) = {b}");
    
    let r2: &i32 = &*x;      // r2 points to the heap value directly
    let c: i32 = *r2;    // so only one dereference is needed to read it
    println!("\n let r2: &i32 = &*x; ");
    println!(" let c: i32 = *r2; ");
    println!(" Value of c (c = *r2) = {c}");
}

/**
 * 3. How Rust Avoids Simultaneous Aliasing and Mutation
 * > Aliasing is accessing the same data through different variables
 * > Aliasing is two or more references point to the same memory location.
 * 
 * L0 -: v points to an array with 3 elements on the heap. 
 * L1 -: Then num is created as a reference to the third element
 * L2 -: However, the operation v.push(4) resizes v. 
 * L3 -: The resize will deallocate the previous array and allocate a new, bigger array. 
 *      In the process, num is left pointing to invalid memory. 
 *      Therefore at L3, dereferencing *num reads invalid memory, causing undefined behavior.
 */
fn fn_simultaneous_aliasing_and_mutation(){
    let mut v: Vec<i32> = vec![1, 2, 3];// L0 -: v points to an array with 3 elements on the heap. 
    let _num: &i32 = &v[2]; //  -: create a num is as a reference to the third element of vector v. data is aliasing
    v.push(4); //L2 -: this reallocates memory because the current capacity is exceeded. 
    //println!("Third element is {}", *num); //L3 -: dereferencing *num reads invalid memory, causing undefined behavior.
}
