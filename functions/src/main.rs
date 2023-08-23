fn fun_b() {
    println!("Definition before main");
}

fn main() {
    fun_b();
    fun_a();
    fun_w_parameters(69);
    println!("Five is: {}",returns_five());
}

fn fun_a() {
    println!("Definition after main");
}

fn fun_w_parameters(x: i32) {
    println!("x is equal to: {}", x);
}

fn returns_five() -> i32{
    5
}
