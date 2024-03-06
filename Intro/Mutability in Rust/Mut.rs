//we use mut to make a variable mutable in rust

fn main(){
    let mut x = 1;
    println!("Value of x = {}", x);

    // change the value of variable x
    x = 2;
    println!("Updated value of x = {}", x);
}