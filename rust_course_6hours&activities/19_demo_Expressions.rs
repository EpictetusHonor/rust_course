//para un servidor se puede usar esto para el acceso, es interesante la verdad
enum Access {
    Admin,
    Manager,
    User,
    Guest
}

fn main (){
    // secret_file = admins only
    let access_level = Access::Admin;
    let can_access_file = match access_level {
        Access::Admin => true,
        _ => false,
    };
    if can_access_file {
        println!("Access Granted.");
    }else{
        println!("Access Denied.");
    }
}