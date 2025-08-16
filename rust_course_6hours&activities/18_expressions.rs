fn main(){
    let my_num = 4;
    let is_5 = if my_num==5 {
        true
    }else{
        false
    };
    // ↑ ↑ ↑  <=>  ↓ ↓ ↓ (They are the same expression!Choose being carefully). 
    let is_5 = my_num == 5;
    println!("{:?}",is_5);
    let my_num = 2;
    let message = match my_num {
        1 => "hello",
        2 => "goodbye",
        _ => "wtf?"
    };
    println!("{:?}",message);
}

//nest expressions


