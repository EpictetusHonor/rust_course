enum Mouse {
    LeftClick,
    RightClick,
    MiddleClick,
    Scroll(i32),
    Move(i32,i32)
}
enum PromoDiscount { // caracteristicas
    NewUser,
    Holiday(String),
}
enum Discount {
    Percent(f64),
    Flat(i32),
    Promo(PromoDiscount), // caracteristica dentro de una caracteristica.
    Custom(String), // cuando crees la variante vas a requerir tener escrito la categoria.
}
struct Ticket {
    event: String,
    price: i32,
}

fn main() {
    let n = 3;
    match n {
        3 => println!("Three"),
        other => println!("number: {:?}", other),
    }
    let flat = Discount::Flat(2);
    match flat {
        Discount::Flat(2) => println!("Flat of '2'"),
        Discount::Flat(amount) => println!("flat discount of: {:?}",amount),
        _ => (),
    }
    let concert = Ticket {
        event : "concert".to_owned(),
        price : 30,
    };
    match concert {
        Ticket{price: 50, event} => println!("Event at 50$ = {:?}", event),
        Ticket{price, ..} => println!("Price = {:?}$",price),
    }
}
