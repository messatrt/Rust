fn main() {
    let x = 5;
    let x = 6;
    {
        let x = x*2;
        println!("The value of x is:{x}");
    }
    println!("The value of x is:{x}")
}
