enum SpreadSheetCell{
    Int(i32),
    Float(f64),
    Text(String)
}

use std::collections::HashMap;

fn main() {
    // CREAZIONE
    let _v: Vec<i32> = Vec::new();
    //La type annotation è necessaria poichè bisogna in qualche modo specificare il tipo, e a
    //creazione esso non può essere dedotto tramite inferenza

    let mut v = vec![1,2,3];
    //macro che crea un vettore partendo da un array (fa automaticamente inferenza di tipo)
    
    // AGGIUNTA
    v.push(4);
    v.push(5);
    v.push(6); //il vettore deve essere mutabile per riuscire ad utilizzare il metodo push

    // LETTURA
    //let third_elem: &i32 = &v[2]; //usiamo l'indexing
    //println!("The third element is {}",third_elem);

    match v.get(2){
        Some(third) => println!("The third element is {}",third),
        None => println!("Element not found")
    }

    v.push(7); //non può essere aggiunto poichè non possiamo avere una reference mutabile
    match v.get(2){
        Some(third) => println!("The third element is {}",third),
        None => println!("Element not found")
    }
    // successiva a delle reference non mutabili
    //println!("The third element is {}",third_elem);
    
    // ITERAZIONE
    for i in &v{
        println!("{}",i);
    }

    for i in &mut v{
        *i += 50;
    }

    let row = vec![SpreadSheetCell::Int(3),SpreadSheetCell::Text(String::from("blue")),SpreadSheetCell::Float(3.14)];
    for i in &row{
        match i{
            SpreadSheetCell::Int(num) => println!("Number is: {}",num),
            SpreadSheetCell::Float(num) => println!("Float is: {}",num),
            SpreadSheetCell::Text(s) => println!("Text is: {}",s),
        }
    }

    // CREAZIONE
    let mut st= String::new();
    let _st_from_str = "string literals".to_string();
    let _st_from_str_w_constructor = String::from("string literals");

    // AGGIORNAMENTO
    st.push_str(" aggiunta");

    let s2 = "hello";
    st.push_str(s2); //non possiamo passare una stringa, ma solo una string literal
    println!("s2 is {}",s2);

    st.push('l'); //per aggiungere caratteri
    
    // CONCATENAZIONE
    let s2 = s2.to_string();
    let _s3 = st + &s2; // la firma del metodo prevede che si passi il primo elemento per ownership
    // e il secondo per reference
    // ORA INFATTI st NON PUO' PIÙ ESSERE UTILIZZATO

    let s1 = String::from("ci");
    let s2 = String::from(" vediamo ");
    let s3 = String::from("!");

    let s4 = s1 + &s2 + &s3;
    println!("{}",s4);

    let s1 = String::from("ci");
    let s4 = format!("{}{}{}",s1,s2,s3);
    println!("{} with format",s4);

    for c in s2.chars(){
        println!("{}",c);
    }
    for c in s2.bytes(){
        println!("{}",c);
    }

    // CREAZIONE
    let mut scores = HashMap::new();

    // INSERIMENTO
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);

    let teams = vec![String::from("blue"),String::from("yellow")];
    let initial_scores = vec![10,50];

    let scores2: HashMap<&String,&i32> = teams.iter().zip(initial_scores.iter()).collect();

    // LETTURA
    let _blue_score = scores.get("blue");
    match scores.get("blue"){
        Some(score) => println!("{}",score),
        None => println!("no score")
    }

    for (key,value) in &scores {
        println!("k: {}, v: {}",key,value);
    }

    // MODIFICA
    scores.insert(String::from("blue"),60); //il blue di prima viene sovrascritto
    
    println!("{:?}",scores);

    scores.entry(String::from("blue")).or_insert(50); //il valore blues è già presente e quindi non
    //inserirà niente
    scores.entry(String::from("green")).or_insert(70); //il valore di green non esiste e quindi
    //viene sovrascritto con 70

    println!("{:?}",scores);

    let count_chars = "3aaa2bb1c";
    let mut chars_counter: HashMap<char, i32> = HashMap::new();

    for ch in count_chars.chars(){
        let count = chars_counter.entry(ch).or_insert(0);
        *count += 1;
    }

    println!("{:?}",chars_counter);
}
