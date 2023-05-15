fn main() {
    let x = 5;

    //the change of x does not affect the one in the outer scope! 
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}
