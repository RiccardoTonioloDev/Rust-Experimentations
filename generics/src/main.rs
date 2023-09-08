fn main() {
    let number_list = vec![32,1,49,599,345,3];
    let result = largest(&number_list);

    println!("The largest number is {}", result); 

    let p1 = Point{x:5,y:9};
    let p2 = Point{x:4,y:7.0}; 
    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}",p3.x,p3.y);
}

//GENERIC function
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
    let mut largest = list[0];
    for &item in list.iter(){
        if item > largest{
            largest = item;
        }
    }
    largest
}

//GENERIC struct
struct Point<T,U> {
    x: T,
    y: U //Ne mettiamo due, di modo che possa essere usato i32 e f64 per lo stesso punto nelle due
    //coordinate
} 
//GENERIC method definition
impl<T,U> Point<T,U>{
    fn x(&self) -> &T{
        &self.x
    }

    //Dichiariamo un metodo generico per un tipo generico
    fn mixup<V,W> (self,other: Point<V,W>) -> Point<T,W>{
        Point{
            x: self.x,
            y: other.y
        }
    }
}

impl Point<f32,f32>{
    fn distance_from_origin(&self) -> f32{
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}
