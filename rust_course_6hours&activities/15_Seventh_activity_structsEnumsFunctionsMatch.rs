// Tema: Organizar datos similares usando structs
//
// Requisitos:
// * Imprimir el sabor de una bebida y sus onzas líquidas
//
// Notas:
// * Usar un enum para crear diferentes sabores de bebidas
// * Usar un struct para almacenar la información del sabor de la bebida y las onzas líquidas
// * Usar una función para imprimir el sabor y las onzas de la bebida
// * Usar una expresión match para imprimir el sabor de la bebida
enum Flavour {
    Pineapple,
    Sweet,
}
struct Drink {
    flav:Flavour,
    ounces:f64,
}
fn print_flavour(flavour:Flavour){
    match flavour {
        Flavour::Pineapple => println!("It's Pineapple!"),
        Flavour::Sweet => println!("It's Sweet!"),
    }
}
fn print_drink(drink:Drink){
    print_flavour(drink.flav);
    println!("Oz: {:?}", drink.ounces);
}
fn main(){
    let pineapple = Drink {
        flav: Flavour::Pineapple,
        ounces: 4.0,
    };
    let sweet = Drink {
        flav: Flavour::Sweet,
        ounces: 6.0,
    };
    print_drink(pineapple);
    print_drink(sweet);
}