/*
Slice is a kind of reference which let us reference a contiguous sequence of elements in a collection 
rather than the whole collection. 
It is a non-owning pointer.

*/

pub fn fn_learn_slice_type(){
    println!("Learn Slices Types");
    println!("==============================================");

    println!("\n******Demo Function for String Slices*********");
    learn_string_slices();

    println!("\n******Demo String Slices: How to use it to get first word*********");
    let s = String::from("Hello World!");
    let first_word = get_first_word(&s);
    println!("Word : {s}");
    println!("First Word: {first_word}");


    println!("\n******Demo Function for Other Slices*********");
    other_slices();
}

/**
 * A string slice is a reference to part of a String, and it looks like this:
 * Rather than a reference to the entire String (like s2), hello is a reference to a portion of the String, 
 *  specified in the extra [0..5] bit. 
 * We create slices using a range within brackets by specifying [starting_index..ending_index], 
 *  where starting_index is the first position in the slice and ending_index is one more than the last position in the slice.
 */
fn learn_string_slices(){
    let s = String::from("hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    let s2: &String = &s; //L1

    println!("hello :{hello} ");
    println!("world :{world} ");
    println!("s2 :{s2} ");

}

/**
 * This is for demonstration how slices are used to get first word of the given sentence.
 */
fn get_first_word(s: &String) -> &str{
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[..i]
        }
    }
    &s[..]    
}

/**
 * Slices of array
 */
fn other_slices(){
    let a = [1, 2, 3, 4, 5, 6];
    let slice = &a[1..3];
    println!("{:?}", slice);
    test();
}

fn test(){    
  let s = String::from("hello");
  let s2: &String = &s;
  let s3: &str = &s[..];

  println!("s2 : {s2}");
  println!("s3 : {s3}");

  println!(
    "&String={} &str={}",
    std::mem::size_of::<&String>(),
    std::mem::size_of::<&str>(),
  );

}