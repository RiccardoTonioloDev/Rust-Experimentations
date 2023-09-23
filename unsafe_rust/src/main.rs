static mut COUNTER: u32 = 0;

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let _r3 = address as *const i32;

    unsafe{
        println!("r1 is : {}",*r1);
        println!("r2 is : {}",*r2);
    }

    add_to_count(4);
    unsafe{
        println!("COUNTER: {}",COUNTER);
    }
}

fn add_to_count(inc: u32){
    unsafe{
        COUNTER += inc;
    }
}
