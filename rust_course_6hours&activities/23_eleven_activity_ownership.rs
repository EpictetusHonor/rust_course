// Topic: Ownership

// Requirements:
//* Print out the quantity and id number of a grocery item

// Notes:
//Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity
// * Create a function to display the id number
struct Apple{
    stock:i32,
    id:i32
}
fn quantity(item:&Apple){
    println!("stock: {:?}",item.stock);
}
fn id(item:&Apple){
    println!("id: {:?}",item.id);
}
fn main(){
    let apple = &Apple {
        stock:10,
        id:42334
    };
    quantity(apple);
    id(apple);
}