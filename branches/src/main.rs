fn main() {
    let number = 4;

    if number != 0{
        println!("È un numero diverso da zero.");
    }

    let boolean_parameter = true;

    let number2 = if boolean_parameter{
        1
    }else {
        -1
    };

    println!("bp è {} e quindi number2 è {}", boolean_parameter,number2);
}
