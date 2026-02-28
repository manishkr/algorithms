fn check_plaindrome(s: &str) -> bool{
let mut forward = s.chars();
    let mut backward = s.chars().rev();

    while let (Some(f), Some(b)) = (forward.next(), backward.next()) {
        if f != b {
            return false;
        }
    }
    true
}
fn main() {
    let res = check_plaindrome("MAAM");
    println!("Result {}", res);
}
