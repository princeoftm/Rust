fn hello(){
    println!("Hello world");
}

fn main(){
    hello();
    println!("{}",get());
}


fn get()-> f64{
    return 3.14;
}