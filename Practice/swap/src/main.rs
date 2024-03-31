fn swap<T>(x: &mut T, y: &mut T) {
    std::mem::swap(x, y)
}

fn main() {
    let mut x = String::from("Hello");
    let mut y = String::from("World");

    println!("Before swap: x = {}, y = {}", x, y);
    swap(&mut x, &mut y);
    println!("After swap: x = {}, y = {}", x, y);

    let mut x = 10;
    let mut y = -15;
    println!("Before swap: x = {}, y = {}", x, y);
    swap(&mut x, &mut y);
    println!("After swap: x = {}, y = {}", x, y);
}
