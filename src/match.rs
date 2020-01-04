fn main() {
  let number = 22;

  match number {
    0 => println!("Zore"),
    1...3 => println!("one two three"),
    n @ 4...100 => println!("greater then three, number is {:?}", n),
    _ => println!("default")
  }
}