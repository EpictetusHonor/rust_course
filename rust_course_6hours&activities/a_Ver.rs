struct Punto { x: i32, y: i32 }

fn main() {
    let p = Punto { x: 3, y: 6 };
    //let Punto{x,y}= Punto {x:3, y:6};
    //println!("{}", p);   // ❌ Error: no implementa Display
    //println!("{:?}", p); // ✅ Punto { x: 3, y: 6 }, solo con derive (debug)
    //println!("{} {}",x,y); // ✅ 3 6
    println!("{} {}",p.x,p.y);  // ✅ 3 6
}