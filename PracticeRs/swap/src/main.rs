fn swap<T>(a: &mut T, b: &mut T) {
    std::mem::swap(a, b)
}
fn main() {
    let mut a = "Hello";
    let mut b = "World";
    println!("a: {}, b: {}", a, b);
    swap(&mut a, &mut b);
    println!("a: {}, b: {}", a, b);

    let mut a = 11;
    let mut b = 123;
    println!("a: {}, b: {}", a, b);
    swap(&mut a, &mut b);
    println!("a: {}, b: {}", a, b);
}
