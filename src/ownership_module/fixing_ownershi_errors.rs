/**
 * This samples shows what is the issue of the code and how to fix it
 */
pub fn fn_learn_fixing_ownership_errors(){
    println!("\n******Let's Learn How to Fix Ownership Errors*********");
    println!("==============================================");

    println!("\n******Demo Function for Returning Refernce to Stack Error and Solution*********");
    fixing_returning_reference_to_stack_issue();

    println!("\n******Demo Function for Not Enough Permission Error and Solution*********");
    fixing_not_enough_permissions_issue();

    println!("\n******Demo Function for Aliasing and Mutating a Data Structure Error and Solution*********");
    fixing_asliasing_and_mutating_datastructe_issue();

    println!("\n******Demo Function for Fixing or Moving Data Out of Collection Error and Solution*********");
    fixing_copying_or_moving_data_out_of_collection_issue();

    println!("\n******Demo Function for Mutating Tuple Fields Errors and Solutions*********");
    mutating_different_tuple_fields_issue();

    println!("\n******Demo Function for Mutating Array Elements Errors and Solutions*********");
    mutating_different_array_elements_issue();
}

// /**
//  * 1. Fixing an Unsafe Program: Returning a Reference to the Stack
//  * Code with Issue
//  * This code is  returning a reference to the stack
//  * issue is with the lifetime of the referred data
//  **/
// fn return_a_string() -> &String {
//     let s = String::from("Hello world");
//     &s
// }

/**
 * This is how above issues is fixed
 * If you want to pass around a reference to a string,
 *   you have to make sure that the underlying string lives long enough.
 */
fn fixing_returning_reference_to_stack_issue(){
    let s1 = return_a_string1();
    println!("s -: {}", s1);

    let s2 = return_a_string2();
    println!("s -: {}", s2);
    
    let s3 = return_a_string3();
    println!("s -: {}", s3);

    let mut s4 = String::from("Hello");
    return_a_string4( &mut s4);
    println!("s -: {}", s4);
}


/**
 * 1. Solution 1
 * One is to move ownership of the string out of the function, changing &String to String
 */
fn return_a_string1() -> String {
    println!("\n******Solution 1 : move ownership of the string out of the function*********");
    let s = String::from("Hello world");
    s
}
/**
 * 2. Solution 2
 * return a string literal, which lives forever (indicated by 'static). 
 * This solution applies if we never intend to change the string, and then a heap allocation is unnecessary
 */
fn return_a_string2() -> &'static str {
    println!("\n******Solution 2 : return a string literal, which lives forever*********");
    "Hello world"    
}

/**
 * 3. Solution 3
 * defer borrow-checking to runtime by using garbage collection
 * Rc::clone only clones a pointer to s and not the data itself. 
 * At runtime, the Rc checks when the last Rc pointing to data has been dropped, and then deallocates the data.
 */
use std::rc::Rc;
fn return_a_string3() -> Rc<String> {
    println!("\n******Solution 3 : defer borrow-checking to runtime by using garbage collection*********");
    let s = Rc::new(String::from("Hello world"));
    Rc::clone(&s)
}

/**
 * 4. Solution 4
 * another possibility is to have the caller provide a "slot" to put the string using a mutable reference
 * the caller is responsible for creating space for the string. This style can be verbose, but it can also be more memory-efficient if the caller needs to carefully control when allocations occur.
 */
fn return_a_string4(output: &mut String) {
    println!("\n******Solution 4 : ave the caller provide a 'slot' to put the string using a mutable reference*********");
    output.replace_range(.., "Hello world");
}

/**
 * Fixing an Unsafe Program: Not Enough Permissions
 * Another common issue is trying to mutate read-only data, or trying to drop data behind a reference.
 */
/*
This function is supposed to create a person's full name from a vector of name parts, including an extra title
This program is rejected by the borrow checker because name is an immutable reference, but name.push(..) requires the W permission 
 */
// fn stringify_name_with_title(name: &Vec<String>) -> String {
//     name.push(String::from("Esq."));
//     let full = name.join(" ");
//     full
// }

fn fixing_not_enough_permissions_issue(){
    let mut name = vec![String::from("Ferris")];
    
    println!("\nSolution 1 : change the type of name from &Vec<String> to &mut Vec<String>*");
    let first1 = stringify_name_with_title1(&mut name);
    println!("name: {:?}", name);
    println!("first1: {first1}");

    println!("\nSolution 2 : take ownership of the name, by changing &Vec<String> to Vec<String>");
      let first2 = stringify_name_with_title2(name);
    //println!("name: {:?}", name); // name cannot be accessed because ownership has been moved
    println!("first2: {first2}");

    println!("\nSolution 3 :   the choice of &Vec is actually a good one, which we do not want to change.
    clone the input name");  
    let mut name = vec![String::from("Ferris")];
    let first3 = stringify_name_with_title3(&name);
    println!("name: {:?}", name);
    println!("first3: {first3}");

    println!("\nSolution 4 :  slice::join already copies the data in name into the string full.");  
    let mut name = vec![String::from("Ferris")];
    let first4 = stringify_name_with_title4(&name);
    println!("name: {:?}", name);
    println!("first3: {first4}");


}


/**
 * 1. Solution 1
 * One straightforward solution is to change the type of name from &Vec<String> to &mut Vec<String>
 * But this is not a good solution! Functions should not mutate their inputs if the caller would not expect it. 
 */
fn stringify_name_with_title1(name: &mut Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

/**
 * 2. Solution 2
 * take ownership of the name, by changing &Vec<String> to Vec<String>
 * But this is also not a good solution! It is very rare for Rust functions to take ownership of heap-owning data structures like Vec and String
 */
fn stringify_name_with_title2(mut name: Vec<String>) -> String {
    name.push(String::from("Esq."));
    let full = name.join(" ");
    full
}

/**
 * 3. Solution 3
 * &Vec is actually a good one, which we do not want to change
 * Here, we can change the body of the function.
 * There are many possible fixes which vary in how much memory they use. 
 * One possibility is to clone the input name:
 */
fn stringify_name_with_title3(name: &Vec<String>) -> String {
    let mut name_clone = name.clone();
    name_clone.push(String::from("Esq."));
    let full = name_clone.join(" ");
    full
}

/**
 * 4. Solution 4
 * By cloning name, we are allowed to mutate the local copy of the vector.
 * However, the clone copies every string in the input. 
 * We can avoid unnecessary copies by adding the suffix later:
 */
fn stringify_name_with_title4(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str(" Esq.");
    full
}

/**
 * Fixing an Unsafe Program: Aliasing and Mutating a Data Structure
 * Another unsafe operation is using a reference to heap data that gets deallocated by another alias.
 */
/*
This function that gets a reference to the largest string in a vector, 
and then uses it while mutating the vector:🐞
This program is rejected by the borrow checker because let largest = .. removes the W permissions on dst. 
However, dst.push(..) requires the W permission. 
Again, we should ask: why is this program unsafe? Because dst.push(..) could deallocate the contents of dst, 
    invalidating the reference largest.
 */
// fn add_big_strings(dst: &mut Vec<String>, src: &[String]) {
//     let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();
//     for s in src {
//         if s.len() > largest.len() {
//             dst.push(s.clone()); //this could deallocate the contents of dst, invalidating the reference largest.
//         }
//     }
// }

fn fixing_asliasing_and_mutating_datastructe_issue(){
    println!("\nSolution 1 :  clone largest.");  
    let mut dst1 = vec![String::from("Ferris"), String::from("Hello")];
    let src = vec![String::from("Jhon"), String::from("Peter")];
    add_big_strings1(&mut dst1, &src);
    println!("{:?}", dst1);

    println!("\nSolution 2 :  perform all the length comparisons first, and then mutate dst afterwards");  
    let mut dst2 = vec![String::from("Ferris"), String::from("Hello")];
    let src = vec![String::from("Jhon"), String::from("Peter")];
    add_big_strings2(&mut dst2, &src);
    println!("{:?}", dst2);

    println!("\nSolution 3 :  copy out the length of largest, since we don't actually need the contents of largest, just its length");  
    let mut dst3 = vec![String::from("Ferris"), String::from("Hello")];
    let src = vec![String::from("Jhon"), String::from("Peter")];
    add_big_strings3(&mut dst3, &src);
    println!("{:?}", dst3);


}

/**
 * Solution 1 :  clone largest
 * This may cause a performance hit for allocating and copying the string data.
 */
fn add_big_strings1(dst: &mut Vec<String>, src: &[String]) {
    let largest: String = dst.iter().max_by_key(|s| s.len()).unwrap().clone();
    for s in src {
        if s.len() > largest.len() { //since we have taken the clone largest, deallocating the content of dst not affect for largest 
            dst.push(s.clone());
        }
    }
}

/**
 * Solution 2 : perform all the length comparisons first, and then mutate dst afterwards
 * , this also causes a performance hit for allocating the vector to_add.
 */
fn add_big_strings2(dst: &mut Vec<String>, src: &[String]) {
    let largest: &String = dst.iter().max_by_key(|s| s.len()).unwrap();

    //here first get the result by comparing and then push the result to dst
    let to_add: Vec<String> = src.iter().filter(|s| s.len() > largest.len()).cloned().collect(); 
    dst.extend(to_add);
}

/**
 * Solution 3:  copy out the length of largest, since we don't actually need the contents of largest, just its length
 * This solution is arguably the most idiomatic and the most performant
 */
fn add_big_strings3(dst: &mut Vec<String>, src: &[String]) {
    let largest_len: usize = dst.iter().max_by_key(|s| s.len()).unwrap().len(); //getting the length of largest
    for s in src {
        if s.len() > largest_len {
            dst.push(s.clone());
        }
    }
}

/**
 * Fixing an Unsafe Program: Copying vs. Moving Out of a Collection
 * A common confusion for Rust learners happens when copying data out of a collection, like a vector.
 */

// fn copy_string(){
//     let v = vec![String::from("Hello World")];

//     let s_ref = &v[0]; //has read access to element of v[0]

//     // this is not allowed because v own string "Hello World"
//     // When we dereference s_ref, that tries to take ownership of the string from the vector. 
//     // But references are non-owning pointers — we can't take ownership through a reference
//     // What happens here is a double-free.
//     // After executing let s = *s_ref, both v and s think they own "Hello world". 
//     // After s is dropped, "Hello world" is deallocated. 
//     // Then v is dropped, and undefined behavior happens when the string is freed a second time.
//     let s = *s_ref; 
    
// } 

/**
 * This gives solutions to fix 
 */
 fn fixing_copying_or_moving_data_out_of_collection_issue(){

    println!("\nSolution 1 :  avoid taking ownership of the string and just use an immutable reference");  
    copy_vector_element_solution();

    println!("\nSolution 2 :  clone the element of the vector");  
    clone_vector_element_solution();

    println!("\nSolution 3 :  move the string out of the vector");  
    remove_the_string();

 }


 /**
  * Solution 1: avoid taking ownership of the string and just use an immutable reference:
  */
fn copy_vector_element_solution(){
    let v: Vec<String> = vec![String::from("Hello world")];
    let s_ref: &String = &v[0]; 
    println!("s_ref : {s_ref}!");
}

/**
 * Solution 2: clone the data if you want to get ownership of the string while leaving the vector alone
 */
fn clone_vector_element_solution(){
    let v = vec![String::from("Hello world")];
    let mut s = v[0].clone();
    s.push('!');
    println!("Value of s : {s}")
}

/**
 * Solution 3: Vec::remove to move the string out of the vector
 */
fn remove_the_string(){
    let mut v: Vec<String> = vec![String::from("Hello world")];
    let mut s: String = v.remove(0);
    s.push('!');
    println!("{s}");
    assert!(v.len() == 0);
}

/**
 * Fixing a Safe Program: Mutating Different Tuple Fields
 * 
 */
fn mutating_different_tuple_fields_issue(){


 }

 /**
  * Fixing a Safe Program: Mutating Different Array Elements
  */
  fn mutating_different_array_elements_issue(){


  }

 




