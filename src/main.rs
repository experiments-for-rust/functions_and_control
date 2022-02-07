//use std::io;
//use std::io::Write;

fn main() {
    let a=35;
    let b=28;

    println!("average() returns {}",average(a as f32,b as f32)); //use as to modify data type
    println!("average2() returns {}",average2(a,b));
    average3(a,b);
    println!("gcd() returns {}",gcd(a,b));
    array_print();
    let_if();
    loop1();
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
    let array2=[1,1,4,5,1,4];

    for i in array1.iter(){
        println!("array_print() prints {}",i);
    }

    for i in 0..5{
        println!("array_print() prints {}",array1[i]);
    }

    for i in (0..5).rev(){
        println!("array_print() prints {} reversely",array1[i])
    }

    for i in array2.iter(){
        println!("array_print() prints {}",i);
    }
}

fn let_if(){
    let tf= false;

    let n=if tf{
        "TRUE"
    }else if tf{
        "FALSE"
    }else{
        "ERROR"
    };

    println!("n={}",n);
}

fn loop1(){
    let out="114514";

    //echo 5 times in loop

    let mut i=0;

    loop{
        i=i+1;
        println!("loop1 output 5 times in loop: {}",out); 
        if i==5{
            break;
        }
    }
}

//while
