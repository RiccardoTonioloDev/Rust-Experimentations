use std::io;

fn main() {
    //Mutability
    let mut x = 5;
    println!("The value of x is: {}",x);
    x = 6;
    println!("The value of x is: {}",x);
    
    //Shadowing
    let y = 5;
    let y = y + 1;
    let y = y * 2;
    println!("The value of y is: {}",y);

    //Using arrays, summoning a panic error
    let a = [1,2,3,4,5];
    println!("Please enter an array index.");

    let mut index = String::new(); 
    io::stdin().read_line(&mut index).expect("Failed to read line");
    let index: usize = index.trim().parse().expect("Not a number");

    let element = a[index];

    println!(
        "The value of the element at index {} is: {}",
        index,element
    );
}
