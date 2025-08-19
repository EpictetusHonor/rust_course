fn crear(n:i32,n1:i32) -> Vec<i32>{
    vec![n,n1]
}

fn main(){
    let vector = crear(1,2);
    for num in vector{
        println!("{}",num);
    }
}