// options2.rs
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a hint.

// I AM NOT DONE

fn main() {
    let optional_word = Some(String::from("rustlings"));
    let optional_word2 = Some(7);
    // TODO: Make this an if let statement whose value is "Some" type

    if let Some(ref word) = optional_word {
        println!("The word is: {}", word);
        println!("{:?} unwraps to: {:?}", optional_word2, optional_word2.unwrap());
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(integer) = optional_integers_vec.pop() {
        println!("current value: {:?}", integer);
    }
}
