use std::ops::Add;

#[derive(Debug,PartialEq)]
struct Point{
    x: i32,
    y: i32
}

impl Add for Point{
    type Output = Self;
    //Abbiamo specificato che il tipo associato output è Point in quanto l'implementazione per
    //Point se usa self con self intende proprio Point.

    //Nell'output stiamo specificando che l'output è Self::Output => Self => Point
    fn add(self, rhs: Self) -> Self::Output {
        Point{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        } 
    }
}

struct Millimiters(u32);
struct Meters(u32);

impl Add<Meters> for Millimiters{
    type Output = Millimiters;

    fn add(self, rhs: Meters) -> Self::Output {
        Millimiters(self.0 + (rhs.0*1000))
    }
}

fn main() {
    assert_eq!(Point{x:1,y:0} + Point{x:2,y:3},Point{x:3,y:3});
}
