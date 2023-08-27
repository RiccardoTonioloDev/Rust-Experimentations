#[derive(Debug)] //usando questa macro possiamo andare a fare il print degli elementi della
//struttura in una maniera conveniente agli sviluppatori, per debuggare i contenuti.
struct Rectangle{
    width: u32,
    height: u32,
}

//qui andiamo a implementare metodi per la struttura
impl Rectangle {
    fn area(&self)->u32{ //il primo parametro è sempre &self (non serve specificare il tipo, perchè
        //Rust conosce già che il tipo di self è Rectangle)
        self.width * self.height
    }
    fn can_hold(&self,second_rectangle: &Rectangle) -> bool{
        self.width > second_rectangle.width &&
            self.height > second_rectangle.height
    }

    fn square(size: u32) -> Rectangle{
        Rectangle { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle{
        width: 30,
        height: 50
    };
    let rect2 = Rectangle{
        width: 20,
        height: 40
    };
    
    println!(
        "L'area del rettangolo è: {}",
        rect1.area()
    );
    println!("rect: {:?}",rect1); //la sintassi del {:?} ci permette di effettuare un print in
    //modalità debug, visto che abbiamo implementato (derivandolo) il trait Debug, per la struttura
    //Rectangle
    println!("rect: {:#?}",rect1); //aggiungere # ci permette di avere una formattazione indentata
    
    println!("Does {:?} hold {:?}? {}",rect1,rect2,rect1.can_hold(&rect2));
    println!("Does {:?} hold {:?}? {}",rect2,rect1,rect2.can_hold(&rect1));
}
