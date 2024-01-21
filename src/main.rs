use core::num;
use std::io;
mod tools;
fn main() {

    let name: String = get_name();
    let university: String = get_college();
    let mut money: i32 = 20_000;
    // generate students
    let mut num_of_students: i32 = 0;
    let mut avg_gpa: f32 = 3.2;

    welcome(&name, &university, &money, &num_of_students, &avg_gpa);

    let title: String = format!("Prez {} of {} University", &name, &university);
    let row_one: String = format!("Funds: ${}", &money);
    let row_two: String = format!("# of Students: {}", &num_of_students);
    let row_three: String = format!("Avg GPA: {}", &avg_gpa);

    println!("{}", tools::pad(title, false));
    
    println!("----------------------------------------------------------------------------------------------");
    // println!("|                                                                                            |");
    // println!("|                                                                                            |");
    // println!("|                                                                                            |");

    println!("{}", tools::pad(row_one, true));
    println!("{}", tools::pad(row_two, true));
    println!("{}", tools::pad(row_three, true));
    println!("----------------------------------------------------------------------------------------------");
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

fn welcome(name: &String, uni: &String, money: &i32, stu: &i32, gpa: &f32) {
    println!("Hey, {}! You have recently been chosen to become the president of {} University", name, uni);
    println!("There are currently: ");
    println!("  ${} in university funds", money);
    println!("  {} students currently enrolled", stu);
    println!("  They have an average gpa of {}", gpa);
    println!("\n    We hope you will develop the university into a prominent, impactful community.")
}