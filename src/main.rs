fn sum(a:i32,b:i32) -> i32 {
    a + b
}
fn display (result:i32) {
    println!("{:?}",result);
}
fn main() {
    let result:i32 = sum(3,4);
    display(result);
}
