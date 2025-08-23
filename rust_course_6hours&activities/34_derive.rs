#[derive(Debug,Clone,Copy)]
enum Trabajade {
    Albañil,
    Aeronautica,
    Electricista
}
#[derive(Debug,Clone,Copy)]
struct Trabajador {
    posicion:Trabajade,
    esclavos:i32,
}
fn println(trab:Trabajador){
    println!("{:?}",trab);
}
fn main(){
    let me = Trabajador {
        posicion: Trabajade::Albañil,
        esclavos: 4,
    };
    println(me);
    println(me);
}