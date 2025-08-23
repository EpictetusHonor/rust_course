// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event *
// * Tickets can be Backstage, Vip, and Standard *
// * Backstage and Vip tickets include the ticket holder's name *
// * All tickets include the price *
// 
// Notes:
// * Use an enum for the tickets with data associated with each variant *
// * Create one of each ticket and place into a vector *
// * Use a match expression while iterating the vector to print the ticket info *

// my analisys and resolution:

// Lista de tickets y su información (Asumo que la lista viene dada por enum);
// y los tickets se dan ;
// los tickets backstage and vip incluyen un ticket holder name
// todos los tickets incluyen precios
enum Tickets {
    Backstage(String),
    Vip(String),
    Standard,
}
struct Ticket{
    price:i32,
    ticket_type:Tickets,
}
impl Ticket {
    fn create_ticket(p: i32,t:Tickets) -> Self {
        Self {
            price: p,
            ticket_type: t,
        }
    }
    fn create_vector(t1: Ticket, t2: Ticket, t3: Ticket) -> Vec<Ticket> {
        vec![t1,t2,t3]
    }
}
fn print_vector(vec:&Vec<Ticket>) {
        for tickets in vec {
            match &tickets.ticket_type {
                Tickets::Standard => println!("Ticket Standard: {:?}$",tickets.price),
                Tickets::Vip(name) => println!("Ticket Vip {:?}: {:?}$",name,tickets.price),
                Tickets::Backstage(name) => println!("Ticket Backstage {:?}: {:?}$",name,tickets.price),
            }
        }
    }
fn main(){
    // tickets types.
    let price = 30;
    let ticket_type = Tickets::Standard;
    let price1 = 70;
    let ticket_type1 = Tickets::Vip("Special".to_owned());
    let price2 = 100;
    let ticket_type2 = Tickets::Backstage("Most Expensive".to_owned());

    // tickets total
    let ticket = Ticket::create_ticket(price,ticket_type);
    let ticket1 = Ticket::create_ticket(price1,ticket_type1);
    let ticket2 = Ticket::create_ticket(price2,ticket_type2);
    
    // tickets vector
    let vector_tickets = Ticket::create_vector(ticket,ticket1,ticket2);

    // prints vectors
    print_vector(&vector_tickets);
}






