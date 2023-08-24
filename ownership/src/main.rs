fn main() {
    //s not valid here
    {
        let _s = "hello"; //string literal (hard coded string value)
        //s valid here
    }//s is out of scope here

    let mut s = String::from("hello");
    s.push_str(", world!"); //possiamo fare l'append di string litterals
    println!("{}",s);


    //interazione tra variabili e valori
    let x = 5;
    let _y = x; //questo ha il comportamento che ci aspettiamo poichè gli interi sono valori
    //semplici e allocabili a compile time.

    let s1 = String::from("hello");
    let s2 = s1; //Qui stiamo passando il riferimento della stringa in memoria da s1 a s2.
    //s1 ora s1 non contiene più niente, poichè il valore è stato spostato in s2.
    //In questo modo quando verrà chiamato il drop al termine dello scope, esso verrà chiamato solo
    //su s2, in quanto s1 non ha più nulla da liberare.
    
    //questo infatti non si può fare: println!("{}",s1);

    let s1 = s2.clone(); //ora però s1 e s2 avranno due copie distinte dello stesso valore, e
    //quindi saranno entrambi stampabili
    println!("s1 = {} s2 = {}", s1, s2);

    takes_ownership(s1);
    //non posso più utilizzare s1 ora, infatti questo non è possibile:
    //println!("{}",s1);
    
    let s1 = gives_ownership();
    takes_ownership(s2);
    let _s2 = takes_and_gives_back_ownership(s1);
}

fn takes_ownership(some_string: String){ //some_string entra nello scope della funzione
    println!("{}",some_string);
} //viene chiamato il drop su tale stringa

fn gives_ownership() -> String{
    String::from("hello")
}

fn takes_and_gives_back_ownership(some_string: String)-> String{
    some_string
}
