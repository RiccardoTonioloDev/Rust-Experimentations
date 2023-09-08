use std::fs::File;
use std::io::Result;
use std::error::Error;
use std::io::{ErrorKind,self, Read};

fn main() {
    let f = File::open("hello.txt");

    let f = match f{
        Ok(file) => file,
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt"){
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}",e) 
            },
            other_error => panic!("Problem opening the file: {:?}", other_error)
        } 
    };
}

fn read_username_from_file() -> Result<String> {
    let mut f = File::open("hello.txt")?;

    //let mut f = match f{
    //    Ok(file) => file,
    //    Err(e) => return e //Se troviamo già un errore qui, allora lo ritorniamo subito
    //};
    //Altrimenti continuiamo con l'esecuzione della funzione

    let mut s = String::new();

    match f.read_to_string(&mut s){
        Ok(_) => Ok(s), //Qui con la sintassi da espressione ritorniamo la stringa letta
        Err(e) => Err(e) //Qui con la sintassi da espressione ritorniamo l'errore
    }
}

fn compact_read_username() -> Result<String> {
    let mut s = String::new();
    File::open("hello.txt")? // primo controllo di errore (per apertura)
        .read_to_string(&mut s)?; // secondo controllo di errore (per lettura)
    Ok(s) // Espressione di ritorno
}
