fn main() {
    let a=35;
    let b=57;
    println!("average() returns {}",average(a,b));
}

fn average(x:i32, y:i32)->i32{
    return (x+y)/2;
}
