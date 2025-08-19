// Topic: Vectors

// Requirements:
// * Print 10, 20, "thirty", and 40 in a loop
// * Print the total number of elements in a vector

// Notes:
// * Use a vector to store 4 numbers
// * Iterate through the vector using a for .. in loop
// * Determine whether to print the number or print "thirty" inside * 
// * Use the .len() function to print the number of elements in a vector *


struct Vector {
    vector: Vec<i32>,
}// vector
impl Vector {
    fn create(num1:i32,num2:i32,num3:i32,num4:i32) -> Self{
        Self {
            vector: vec![num1,num2,num3,num4]
        }// crea un nuevo vector con numeros ingresados
    }
    fn print(&self) {
        for num in &self.vector {
            match num {
            30 => println!("Number: thirty"),
            _ => println!("Number: {:?}",num),
            }
        }
    } //println vector, si es 30 lo escribe, sino lo devuelve como numero.
}

fn main(){
    let vec = Vector::create(10,20,30,40);
    vec.print();
    println!("Largo del vector: {:?}",vec.vector.len());
}