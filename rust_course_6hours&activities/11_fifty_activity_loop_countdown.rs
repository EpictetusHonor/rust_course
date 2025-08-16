fn main(){
    let mut my_int:i32 = 0;
    loop{
        my_int = my_int+1;
        if my_int == 5 {
            break;
        }
        println!("{my_int:?}")
    }
}