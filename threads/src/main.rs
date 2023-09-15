use std::thread;
use std::time::Duration;

use std::sync::mpsc;

use std::sync::Mutex;

fn main() {
    let handle = thread::spawn(||{
        for i in 1..10 {
            println!("hi number {} from the spawned thread!",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
        println!("hi number {} from the main thread!",i);
        thread::sleep(Duration::from_millis(1));
    }

    println!("-------------------------------------------");
    let v = vec![1,2,3];

    let handle2 = thread::spawn(move ||{
        println!("Here's a vector: {:?}",v);
    });
    handle2.join().unwrap();

    println!("-------------------------------------------");
    let (tx,rx) = mpsc::channel();

    //Qui stiamo trasmettendo
    thread::spawn(move ||{
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    //Qui stiamo ricevendo
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
    println!("-------------------------------------------");
    let (tx,rx) = mpsc::channel();

    //Qui stiamo trasmettendo
    thread::spawn(move ||{
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread")
        ];
        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    //Qui stiamo ricevendo
    for received in rx{
        println!("Got: {}",received);
    }
    println!("-------------------------------------------");
    let (tx,rx) = mpsc::channel();
    let t1 = mpsc::Sender::clone(&tx);

    //Qui stiamo trasmettendo
    thread::spawn(move ||{
        let vals = vec![
            String::from("T1:hi"),
            String::from("T1:from"),
            String::from("T1:the"),
            String::from("T1:thread")
        ];
        for val in vals{
            t1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    thread::spawn(move ||{
        let vals = vec![
            String::from("T2:hi"),
            String::from("T2:from"),
            String::from("T2:the"),
            String::from("T2:thread")
        ];
        for val in vals{
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    //Qui stiamo ricevendo
    for received in rx{
        println!("Got: {}",received);
    }
    println!("-------------------------------------------");
    
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}",m);
    println!("-------------------------------------------");
    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in (0..10){
        let handle = thread::spawn(move ||{
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles{
        handle.join().unwrap();
    }
}
