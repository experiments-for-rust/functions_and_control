fn main() {
    let a=35;
    let b=28;

    println!("average() returns {}",average(a as f32,b as f32)); //use as to modify data type
    println!("average2() returns {}",average2(a,b));
    average3(a,b);
    println!("gcd() returns {}",gcd(a,b));
    array_print();
}

fn average(x:f32, y:f32)->f32{ //define returned data type; notice the data type
    return (x+y)/2.0;
}

fn average2(x:i32, y:i32)->i32{
    (x+y)/2 //rust-style return
}

fn average3(x:i32, y:i32){ //void return function
    println!("average3() directly prints {}",(x+y)/2); //also works without ;
}

fn gcd(x:i32, y:i32)->i32{
   if y==0{
       return x;
   }else{
       return gcd(y, x%y);
   }
}

fn array_print(){
    let array1:[i32;5]=[1,2,3,4,5];

    for i in array1.iter(){
        println!("array_print() prints {}",i);
    }

    for i in (0..5).rev() {
        println!("array_print() prints {} reversely",array1[i])
    }

    
}

//NEXT: use condition sentence in value assignment
//match, loop, do while
