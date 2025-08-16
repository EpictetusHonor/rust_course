enum Color { //es un type creado, como cuando yo creo registros, similar. en vez de ser integer es Color. Un grupo.
    Red,
    White,
    Orange
}
fn printcolor(my_color:Color){
    match my_color {
        Color::Red => println!("It's Red!"),
        Color::White => println!("It's White!"),
        Color::Orange => println!("It's Orange!"),
    }
}
fn main(){
    printcolor(Color::Red);
}