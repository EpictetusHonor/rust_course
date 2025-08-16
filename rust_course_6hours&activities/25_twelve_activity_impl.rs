// Topic: Implementing functionality with the impl keyword

// Requirements:
// * Print the characteristics of a shipping box *
// * Must include dimensions, weight, and color *

// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics
enum Color {
    Green,
    White,
    Violet
}
impl Color {
    fn print(&self){
        match self{
            Color::White => println!("color: white"),
            Color::Green => println!("color: green"),
            Color::Violet => println!("color: violet")
        }
    }
}// color code.
struct Dimensions {
    width:f64,
    height:f64,
    depth:f64
}
impl Dimensions {
    fn print(&self){
        println!("dimension width: {:?} ^2",self.width);
        println!("dimension depth: {:?} ^2",self.height);
        println!("dimension height: {:?} ^2",self.depth);
    }
}//dimension code.
struct ShippingBox {
    dimension:Dimensions,
    weight:f64,
    color:Color
}
impl ShippingBox {
    fn new_box(dimension:Dimensions,weight:f64,color:Color) -> Self {
        Self {
            dimension,
            weight,
            color,
        }
    }
    fn print_box(&self){
        self.dimension.print();
        println!("weight: {:?} Inches",self.weight);
        self.color.print();
    }
}//shipping box code.
fn main(){
    let dimension = Dimensions {
        width:34.0,
        height:4.4,
        depth:6.0
    };//
    let color = Color::White;//
    let weight = 3.3; // box information

    let shp_bx = ShippingBox::new_box(dimension,weight,color);
    shp_bx.print_box();
}//main code.