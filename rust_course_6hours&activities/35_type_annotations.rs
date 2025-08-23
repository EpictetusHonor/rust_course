fn main(){
    let binario_con_signo:i8 = 4;  //8 bits limited integer
    let binario_con_signo:i16 = 4;  //16 bitsl limited integer
    let binario_con_signo:i32 = 4;   //32 bits limited integer
    let binario_con_signo:i64 = 4;    //64 bits limited integer
    let binario_sin_signo:u8 = 4;  //8 bits limited natural
    let binario_sin_signo:u16 = 4;  //16 bits limited natural
    let binario_sin_signo:u32 = 4;   //16 bits limited natural
    let binario_sin_signo:u64 = 4;    //64 bits limited natural
    let fraccional:f32 = 4.0; //32 bits limited float
    let fraccional:f64 = 5.0;  //64 bits limited float
    let texto_con_propietario:String = "Recipe".to_owned(); // owned propiety string
    let texto_con_propietario:String = String::from("Recipe"); // owned propiety string
    let texto_en_prestamo:&str = "Recipe"; // borrowed string
    let caracter:char = '4'; // caracter
}