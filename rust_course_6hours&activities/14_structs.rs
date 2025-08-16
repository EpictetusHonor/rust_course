struct Grocery{
    stock:i32,
    price:f64,
}
fn main(){
    let apple = Grocery {
        stock:3,
        price:5.5,
    };
    println!("The box is {:?}",apple.stock);
    println!("The box is {:?}",apple.price);
}