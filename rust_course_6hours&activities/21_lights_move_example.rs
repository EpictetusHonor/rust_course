enum Lights {
    Black,
    Green,
}
fn display_lights(light:&Lights){
    match light {
        Lights::Black => println!("Black!"),
        Lights::Green => println!("Green!"),
    }
}
fn main(){
    let green = Lights::Black;
    display_lights(&green);
    display_lights(&green);
}