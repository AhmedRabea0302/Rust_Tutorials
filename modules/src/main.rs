use modules::greet;
 
fn main() {
    let x = rand::thread_rng().gen_range(0, 100);
    greet();
    println!(x);
    println!("Hello, world!");
}
