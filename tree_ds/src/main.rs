use std::cell::RefCell;
use std::rc::{Rc,Weak};

#[derive(Debug)]
struct Node{
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    //La refcell ci permette di mutare il contenuto del vettore.
    //La rc ci permette di andare andare a poter gestire reference multiple dei nodi (le possiamo
    //dichiarare in variabili, e successivamente possiamo condividerle).
    parent: RefCell<Weak<Node>>
}

fn main() {
    let leaf = Rc::new(Node{
        value:3,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new())
    });

    let branch = Rc::new(Node{
        value: 5,
        children: RefCell::new(vec![Rc::clone(&leaf)]),
        parent: RefCell::new(Weak::new())
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
}
