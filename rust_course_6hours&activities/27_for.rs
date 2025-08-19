//for => for
//'i:=1' cambia por 'num' (pone directo el valor)
//to -> in
//40 => 40 o vector: '1,2,3'
//do ->   (no se usa mas)
//begin -> {
//end -> }
//'' -> ""
struct first3 {
    num:i32,
}
fn main(){
    let my_numbers = vec![first3{num:1},first3{num:2},first3{num:3}];
    
    for num in my_numbers { //for in es un loop especifico para colecciones
        println!("{:?}",num.num);        //puede ser leido por alguna variable en la coleccion my_numbers
    }
}