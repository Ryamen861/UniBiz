use std::io;
use std::time::Duration;
use std::time;
use tokio::time::sleep;
use student::StudentManager;
use std::fs;
mod tools;
mod student;
fn main() {
    let name_pool: Vec<String> = get_student_names();

    let mut sm: StudentManager = StudentManager {
        id: 0,
        students: vec![],
    };
    for _ in 0..20_000 {
        // sm.make_student();
    }

    let name: String = get_name();
    let university: String = get_college();
    let mut money: i32 = 20_000;
    let mut num_of_students: i32 = 0;
    let mut avg_gpa: f32 = 3.2;
    let mut day: i32 = 1;

    next_page();
    welcome(&name, &university, &money, &num_of_students, &avg_gpa);
    wait(4);
    next_page();
    
    let mut now = time::Instant::now();

    loop {
        // format strings for dashboard
        let title: String = format!("Prez {} of {} University", &name, &university);
        let row_one: String = format!("Funds: ${}", &money);
        let row_two: String = format!("# of Students: {}", &num_of_students);
        let row_three: String = format!("Avg GPA: {}", &avg_gpa);
        let time: String = format!("Day: {}", &day);
        
        // display dashboard
        println!("{}", tools::pad(title, false));
        println!("----------------------------------------------------------------------------------------------");
        println!("{}", tools::pad(row_one, true));
        println!("{}", tools::pad(row_two, true));
        println!("{}", tools::pad(row_three, true));
        println!("----------------------------------------------------------------------------------------------");
        println!("{}", tools::pad(time, false));

        let action: i32 = get_action();
        if action == 123 {
            break;
            // save changes sequence
        }

        next_page();
    }

}

fn get_action() -> i32{
    loop {
        let mut action: String = String::new();
        io::stdin().read_line(&mut action).expect("Error getting action");

        next_page();
        let action: i32 = match action.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        return action
    }
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

fn next_page() {
    println!("\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n\n");
}

#[tokio::main]
async fn wait(sec: i32) {
    sleep(Duration::from_millis((sec * 1000) as u64)).await;
}

fn get_student_names() -> Vec<String> {
    // fs::read_to_string("pool/names.txt")
}