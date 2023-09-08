use std::collections::BTreeMap;

fn main() {
    let list_of_integers = vec![1,1,1,2,3,3];

    let mut occurrency_counter = BTreeMap::new();
    let mut sum = 0;
    for n in &list_of_integers{
        let counter = occurrency_counter.entry(n).or_insert(0); 
        *counter += 1;
        sum = sum + n;
    }

    let mut median: f64 = 0.0;
    if list_of_integers.len() != 0{
        if list_of_integers.len() % 2 == 0{
            median = 
                (list_of_integers[list_of_integers.len() / 2 -1] as f64 + list_of_integers[(list_of_integers.len()/2)] as f64)/2.0;
        }else{
            median = list_of_integers[list_of_integers.len()/2] as f64;
        }
    }
    println!("The median is: {}",median);
    let max_val = occurrency_counter.values().cloned().max().unwrap_or(0);
    let mode_vec: Vec<&i32>= occurrency_counter
            .into_iter()
            .filter(|&(_,frequency)| frequency == max_val)
            .map(|(value,_)| value)
            .collect();

    println!(
        "The mode is: {:?}",
        mode_vec
    );
    println!("The mean is: {}",sum as f64 / list_of_integers.len() as f64);
}
