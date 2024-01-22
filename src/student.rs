use rand::Rng;
use weighted_rand::{self, builder::WalkerTableBuilder};
pub struct Students {
    students: Vec<Student>,
}
pub struct Student {
    id: i32,
    name: String,
    active: bool,
    major: String,
    year: i8,
    gpa: f32,
    minor: Option<String>

}

pub fn make_students(id: i32, name_pool: &Vec<String>, 
                            major_pool: &Vec<String>, 
                            minor_pool: &Vec<String>,
                            make_new: bool) {
    if !make_new {
        
        // active
        let active_coin = [true, false];
        let active_weights = [900, 1];
        let builder = WalkerTableBuilder::new(&active_weights);
        let table = builder.build();

        

        let new_student: Student = Student {
            id,
            name: name_pool[rand::thread_rng().gen_range(0..name_pool.len())],
            active: ,
            major: 
            year: 
            gpa:,

            }
        }
    } else {
        let new_student: Student = Student {
            id,
            name: 
            active: true,
            major: 
            year: 
            gpa: 4.0,
    
        }
    }

fn random_name() -> String {

}