fn main() {
    let v1 = vec![1,2,3];

    let v1_iter = v1.iter();

    let total: i32 = v1_iter.sum();

    //for item in v1_iter{
    //    println!("Value is {}",item);
    //}
    println!("Total is: {}",total);

    let v2_iter = v1.iter();
    
    let v1_with_strings: Vec<String> = v2_iter.map(|n| n.to_string()).collect();

    println!("Vec of strings is: {:?}", v1_with_strings);
}

