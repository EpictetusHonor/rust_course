// Topic: Working with expressions

// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100

// Notes:
// * Use a boolean variable set to an expression
// that determines whether the value is >100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
// to print
fn print(more_than_100:bool){
    match more_than_100 {
        true =>  println!("its big"),
        false =>  println!("its small")
    }
}

fn main(){
    let number = 100;
    let its_gt_100 = number>100;
    print(its_gt_100);
}