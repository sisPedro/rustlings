// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand for a hint.

// I AM DONE

fn main() {
    let a: [i32;500] = [0;500];

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
        for i in 0..a.len(){
            println!("{}",i)
        }
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
