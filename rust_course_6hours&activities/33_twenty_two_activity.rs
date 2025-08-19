// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for .. in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

// Escribir: nombres y colores fav de personas de menos de 10 años.
// Crear struct de edad de personas, nombre y color fav
// El color y el nombre debera ser almacenado en un string
// Cree y almacene 3 personas en el vector.
// Iterelas en un vector usando el for .. in loop
// Use una expresion if que determine que información debe ser escrita
// El nombre y colores deben ser escritos usando una función.
struct Persons {
    age:i32,
    name:String,
    fav_colour:String
}
fn print(data:&str){
    println!("{}",data);
}

fn main(){
    let people = vec![
        Persons{
            age:3,
            name:"Marcos".to_owned(),
            fav_colour:"Red".to_owned()
        },
        Persons{
            age:30,
            name:"Carlos Tevez".to_owned(),
            fav_colour:"9/12 (es lo unico que ve)".to_owned()
        },
        Persons{
            age:60,
            name:"Marta".to_owned(),
            fav_colour:"White".to_owned()
        }
        ];
    for all in people {
        if all.age <= 10 {
            println!("{}",all.age);
            print(&all.name);
            print(&all.fav_colour);
        }
    }
}
