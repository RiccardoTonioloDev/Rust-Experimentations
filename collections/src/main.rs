enum SpreadSheetCell{
    Int(i32),
    Float(f64),
    Text(String)
}

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
    
    let st_from_str = "string literals".to_string();

    let st_from_str_w_constructor = String::from("string literals");
}
