use std::io;
use std::time::Duration;
use std::time;
use tokio::time::sleep;
mod tools;
fn main() {

    let name: String = get_name();
    let university: String = get_college();
    let mut money: i32 = 20_000;
    // generate students
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

        if now.elapsed().as_secs() > 2 as u64 {
            day += 1;
            now -= now.elapsed();
        } else {
            let mut action: i32;
            let attempt_ask = tokio::time::timeout(Duration::from_secs(1), get_action(&mut action));
                                                                                                                // for tmrw me: figure this out
            // and also, I am getting into async/await/multi threading, maybe avoid altogether
            if action == 123 {
                break;
                // save changes sequence
            }
        }

        next_page();
    }

}

async fn get_action(a: &mut i32){
    loop {
        let mut action: String = String::new();
        io::stdin().read_line(&mut action).expect("Error getting action");

        next_page();
        let action: i32 = match action.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        *a = action;
        break;
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