fn main() {
    //loop
    let mut counter = 0;
    loop {
      println!("Counter: {}",counter);
      counter+=1;
      if counter == 10 {break;}
    }

    //while
    let mut counter = 0;
    while counter != 10 {
      println!("Counter: {}",counter);
      counter+=1;
    }

    //for
    let a = [0,1,2,3,4,5,8,9];
    for element in a.iter(){
      println!("the value is: {}",element);
    }

    for number in (0..10).rev(){ //il range va dal primo incluso all'ultimo escluso, e poi lo
    //inverte
      println!("{}!",number);
    }
}
