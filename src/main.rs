use std::io;
mod tools;
fn main() {

    let name: String = get_name();
    let university: String = get_college();
    println!("Hey, {}! You have recently been chosen to become the president of {} University", name, university);

    println!("                       UniBiz                         ");
    let mut title: String = format!("{} of {}", &name, &university);
    
    
    println!("------------------------------------------------------");
    println!("|                                                    |");
    println!("|                                                    |");
    println!("|                                                    |");
    println!("------------------------------------------------------");
}

fn get_name() -> String {
    let mut name: String = String::new();

    println!("Type in your name:");
    io::stdin().read_line(&mut name).expect("Error receiving name");
    name = name.trim().to_uppercase();
    return name
}

fn get_college() -> String {
    let mut university: String = String::new();
    
    println!("Type in the name of your college (do not write 'college' or 'university' at the end, just the name)");
    io::stdin().read_line(&mut university).expect("Error receiving college");
    university = university.trim().to_uppercase();
    return university
}
