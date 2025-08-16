#[derive(Debug)]
struct Punto { x: i32, y: i32 }

fn main() {
    //let p = Punto { x: 3, y: 6 };
    let Punto{x,y}= Punto {x:3, y:6};
    //println!("{}", p);   // ❌ Error: no implementa Display
    //println!("{:?}", p); // ✅ Punto { x: 3, y: 6 }
    println!("{} {}",x,y);
}