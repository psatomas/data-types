fn main() {
    let is_hansome = true;
    let is_silly: bool = false;

    println!("handsome: {is_handsome}. Silly: {is_silly");

    let age: i32 = -40;
    let is_young = age < 35;
    println!("{is_young}");
    println!("{} {}", age.is_positive(), age.is_negative())
}
