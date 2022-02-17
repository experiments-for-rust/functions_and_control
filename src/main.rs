//use std::io;
//use std::io::Write;

fn main() {
    //let a=35;
    let a={
        let a_temp=34;
        a_temp+1 // return not allowed; returned value here
    };
    let b=28;

    println!("average() returns {}",average(a as f32,b as f32)); //use as to modify data type
    println!("");
    average2(a,b);

    println!("gcd() returns {}",gcd(a,b));
    println!("");

    array_print();
    let_if();

    loop1();
    我趣汉字();
}

fn average(x:f32, y:f32)->f32{ //define returned data type; notice the data type
    //(x+y)/2
    return (x+y)/2.0; //return allowed
}

fn average2(x:i32, y:i32){ //void return function
    println!("average2() directly prints {}",(x+y)/2); //also works without ;
    println!("");
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
    //let array2=[1,1,4,5,1,4];

    for element in array1{
        println!("array_print() prints {}",element);
    }

    for i in 0..5{
        println!("array_print() prints {}",array1[i]);
    }

    for i in (0..5).rev(){
        println!("array_print() prints {} reversely",array1[i])
    }
    println!("");
}

fn let_if(){
    let condition= false;

    let n=if condition{
        "TRUE"
    }else if !condition{
        "FALSE"
    }else{
        "ERROR"
    };

    println!("n={}",n);
    println!("");
}

fn loop1(){
    let out="114514";
    let mut i=0;

    loop{
        i=i+1;
        println!("loop1 output 5 times in loop: {}",out); 
        if i==5{
            break;
        }
    }
    println!("");

    i=0;
    'counting: loop{ //label for loop
        println!("i={}", i);
        let mut j=10;

        loop{
            println!("j={}",j);
            if j==9{
                break;
            }
            if i==2{
                break 'counting;
            }
            j-=1;
        }
        i+=1;
    }
    println!("count endded at i={}", i);
    println!("");

    i=0;
    let r=loop{
        i+=1;
        if i==10{
            break i*2;
        }
    };
    println!("r={}",r);
    println!("");
}

fn 我趣汉字(){
    let mut i=0;
    while i<=10{
        println!("i={}",i);
        i+=1;
    }
    println!("");
}