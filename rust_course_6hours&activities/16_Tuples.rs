enum Access {
    Full
}
fn tuples(x:i32) -> (i32,i32,i32){
    (x+1,4,x)
}
fn precios () -> (&str,i32,&str,i32){
    ("burger",10000,"cocacola",2000)
}

fn write_all(tuple:(i32, i32, i32)){
    println!("{:?}",tuple.0);
    println!("{:?}",tuple.1);
    println!("{:?}",tuple.2);
}
fn main(){
    let num = 4;
    let tot = tuples(num);
    write_all(tot);
    let (numero,_access) = ("4",Access::Full);
    let numerovich = ("4",_access);
    println!("{:?}",numerovich.0);
    let (fav_food, fav_food_price,coca,coca_price) =  precios();
    println!("{:?} {:?} {:?} {:?} ",fav_food,fav_food_price,coca,coca_price);
}