// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// I AM DONE

fn main() {
    let mut vec0 = Vec::new();
    // paso una referencia al vec0. 

    
    // let mut vec1 =
     fill_vec(&mut vec0);
    // let mut vec1 = fill_vec(vec0.to_vec());
    // let mut vec1 = fill_vec(vec0.clone());

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>) {
    let mut vec = vec.to_vec();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    // vec
}
