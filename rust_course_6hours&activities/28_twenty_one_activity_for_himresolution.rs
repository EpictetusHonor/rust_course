fn main(){
    // * Use a vector to store 4 numbers *
    let vector = vec![10,20,30,40];
    // * Iterate through the vector using a for .. in loop *
    for num in &vector {
        match num {
            30 => println!("thirty"),
            _ => println!("{:?}",num),
        }
    }
    println!("Largo del vector: {:?}",vector.len());
// * Determine whether to print the number or print "thirty" inside * 
// * Use the .len() function to print the number of elements in a vector *
}