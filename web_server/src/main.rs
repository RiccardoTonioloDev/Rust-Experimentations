use std::net::{TcpListener, TcpStream}; //per ricevere e stabilire connessioni


use std::thread;
use std::time::Duration;

use std::io::prelude::*; //per riuscire ad usare alcune primitive per leggere e scrivere nello
//stream

use std::fs; //per usare il filesystem di modo da leggere il file html

use web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //bind sarebbe come la classica funzione new
    // - connettere sulla porta 80 andrebbe a fallire probabilmente poichè richiede i permessi di
    // amministratore (da qui la necessità di esprimere un Result come tipo di ritorno)
    
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(2){
        // - incoming ci fornisce un iteratore che ci permette di scorrere una sequenza di stream
        // di tipo TcpStream (un singolo stream rappresenta una singola connessione tra il client
        // e il server)(una connessione è il nome per l'intero processo di richiesta-risposta)
        let stream = stream.unwrap();

        // Questo gestirebbe tutte le connessioni con un thread separato (ci rende vulnerabile ad
        // un attacco di tipo DoS)
        //thread::spawn(|| {
        //    handle_connection(stream); //stiamo passando l'ownership dello stream ad una funzione
        //    //apposita per gestirlo
        //});

        pool.execute(||{
            handle_connection(stream);
        });
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0;1024]; //abbiamo creato un buffer di 1024 bytes, di modo che sia
    //sufficientemente grande per gestire una richiesta in entrata (base) 

    stream.read(&mut buffer).unwrap(); //viene letto lo stream all'interno del buffer

    //println!("Request: {}",String::from_utf8_lossy(&buffer[..]));
    //Convertiamo i byte del buffer in una stringa

    //let response = "HTTP/1.1 200 OK\r\n\r\n"; //formato di risposta (in encoding stringa) che
    //serve per comunicare un successo di connessione

    let get = b"GET / HTTP/1.1\r\n"; //creiamo una byte string per fare il confronto con l'inizio
    //del buffer

    let sleep = b"GET /sleep HTTP/1.1\r\n"; //creiamo una byte string per fare il confronto con l'inizio
    //del buffer

    //implementiamo una specie di logica di routing
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK","hello.html")
    } else if buffer.starts_with(sleep){
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK","hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND","404.html")
    };
    let contents = fs::read_to_string(filename).unwrap();
    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,contents.len(), contents
    ); //formattiamo la risposta nel modo valido HTTP

    stream.write(response.as_bytes()).unwrap();
    //il metodo write prende un &[u8] e manda tali byte direttamente giù per la connessione

    stream.flush().unwrap(); //andrà ad aspettare, finchè tutti i bytes non saranno mandati giù per
    //la connessione

}
