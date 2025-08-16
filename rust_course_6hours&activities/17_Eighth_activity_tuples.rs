fn coordinate () ->(i32,i32){
    (3,7)
}
fn main(){
    let (_x,y) = coordinate();

    if y > 5 {
        println!(" 'Y' value is > 5: {:?}.",y);
    } else if y < 5 {
                println!(" 'Y' value is < 5: {:?}.",y);
    } else {
        println!(" 'Y' value is = 5: {:?}.",y);
    }
}