//will not work
struct Employee {
//    name:&str,
    name:String,
} //PROBLEMA: la estructura hace una limpieza al terminar.
  //con info prestada no puede hacer la limpieza adecuada.
//fn not_work(){
//    let employee_name = "Ramirez";
//    let emp = Employee {
//        name:employee_name,
//    };
//}
//will work properly
fn work(){
    let _employee_name = "Ramarez".to_owned(); //esto transforma: &str (prestado) => String (propietario)
    let employee_name = String::from("Ramarez");
    let emp = Employee {
        name:employee_name
    };
    println!("{:?}",emp.name);
}
fn main(){
    work();
} 