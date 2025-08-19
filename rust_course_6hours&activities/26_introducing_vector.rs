fn main(){
    let _my_numbers = vec![1,2,3]; //first method (macro), most common and probably the selected method
    let mut my_numbers = Vec::new(); //second method
    my_numbers.push(1); // index: 0
    my_numbers.push(2); // index: 1
    my_numbers.push(3); // index: 2
    my_numbers.pop(); // remueve el ultimo elemento del vector, en este caso el 3
    println!("largo del vector: {:?}",my_numbers.len()); // devuelve el numero de elementos del vector
    let one = my_numbers[1]; // devuelve 2
    println!("vector numero 1: {:?}",one);
}   