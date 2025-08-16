struct Temperatura {
    grados_f:i32
}
impl Temperatura { //implementaciones, es un lugar en el que se almacenan funciones 
//                   que solo se usaran en estructuras. su implementacion es sencilla.. pones un punto en la struct. (algo.funcion).
// su uso viene derivado a la practicidad, orden y agrupacion de funciones especificas.
    fn dibuja_temp(&self){
        println!("grados: {:?}",self.grados_f);
    }
    fn freezing() -> Self {//crea un nuevo "struct" temperature, no esta relacionado. 
        Self {grados_f:0}
    }
    fn boiling() -> Self {
        Self{grados_f:100}
    }
} // :: se usa para tomar cosas de las implementaciones
fn main(){
    let normal_temp = Temperatura{
        grados_f:30,
    };
    normal_temp.dibuja_temp();  //esta es la implementacion ideal.
    //Temperatura::dibuja_temp(&temp);   esta es la forma complicada y sinsentido, claramente usare la primera.
    
    let cold = Temperatura::freezing(); //aca no me queda de otra! xD es medio confuso esto wachin
    cold.dibuja_temp();
    
    let boiling = Temperatura::boiling();
    boiling.dibuja_temp();
}