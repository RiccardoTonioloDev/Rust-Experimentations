fn main() {
    let mut s1 = String::from("hello");
    //passaggio di una reference di s1
    let len = calculate_length(&mut s1);
    let len2 = calculate_length(&mut s1);

    println!("s1 = {} with len {}", s1, len);
}

//Notare che qui stiamo passando una stringa per reference
fn calculate_length(s: &mut String) -> usize{
    s.len()
}
