use crate::List::{Cons,Nil};
use std::ops::Deref;
use std::rc::Rc;

fn main() {
    let b = Box::new(5);

    println!("b = {}", b);

    let a = Rc::new(Cons(1,
        Rc::new(Cons(2, 
            Rc::new(Cons(3, 
                Rc::new(Nil)
            ))
        ))
    ));

    let b = Cons(4, Rc::clone(&a));
    let c = Cons(5, Rc::clone(&a));

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5,x);
    assert_eq!(5,*y);

    let name = MyBox::new(String::from("Rust"));
    let ref_name = &name;
    let str = ref_name;
    hello(&name);

    let c = CustomSmartPointer{data:String::from("my stuff")};
    let d = CustomSmartPointer{data:String::from("other stuff")};
    println!("CustomSmartPointers created.");
}

enum List{
    Cons(i32,Rc<List>),
    Nil
}

struct MyBox<T>(T);
impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T>{
    type Target = T;

    fn deref(&self) -> &Self::Target {
       &self.0 
    }
}
fn hello(name: &str){
    println!("Hello, {}!", name);
}

struct CustomSmartPointer{
    data: String
}
impl Drop for CustomSmartPointer{
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data {}",self.data);
    }
}
