fn main() {
    let a=35;
    let b=57;
    println!("average() returns {}",average(a,b));
    println!("average2() returns {}",average2(a,b));
}

fn average(x:i32, y:i32)->i32{ //define return data type
    return (x+y)/2;
}

fn average2(x:i32, y:i32)->i32{
    (x+y)/2 //rust-style return
}