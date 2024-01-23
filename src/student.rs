use rand::Rng;
use weighted_rand::builder::*;

pub struct StudentManager {
    pub id: i32,
    pub students: Vec<Student>,
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


impl StudentManager {
    fn bump_id(&mut self) {
        self.id += 1;
    }

    pub fn make_student(&mut self, name_pool: &Vec<String>, 
                major_pool: &Vec<String>, 
                minor_pool: &Vec<String>,
                incoming_stu: bool) {
        
        self.bump_id();

        let new_student: Student;
        if !incoming_stu {

            // active //// can also use rand::distributions;
            let active_coin: [bool; 2] = [true, false];
            let active_weights = [900, 1];
            let active_builder = WalkerTableBuilder::new(&active_weights);
            let active_table = active_builder.build();

            let mut rng = rand::thread_rng();
            let index = active_table.next_rng(&mut rng);

            new_student = Student {
                id: self.id.clone(),
                name: name_pool[rand::thread_rng().gen_range(0..name_pool.len())].clone(),
                active: active_coin[index],
                major: major_pool[rand::thread_rng().gen_range(0..name_pool.len())].clone(),
                minor: self.minor_decision(&minor_pool),
                year: rand::thread_rng().gen_range(1..5),
                // can also use rand::distributions::WeightedIndex;
                gpa: rand::thread_rng().gen_range(1.0..4.1),
            };
        } else {
            new_student = Student {
                id: self.id.clone(),
                name: name_pool[rand::thread_rng().gen_range(0..name_pool.len())].clone(),
                active: true,
                major: major_pool[rand::thread_rng().gen_range(0..name_pool.len())].clone(),
                minor: self.minor_decision(&minor_pool),
                year: 1,
                gpa: 4.0,
            };
        }

        println!("name: {}", &new_student.name);
        self.students.push(new_student)
    }

    fn minor_decision(&self, pool: &&Vec<String>) -> Option<String>{
        let coin: [bool; 2] = [true, false];
        let weights = [1, 3];
        let builder: WalkerTableBuilder = WalkerTableBuilder::new(&weights);
        let table = builder.build();

        let mut rng = rand::thread_rng();
        let index = table.next_rng(&mut rng);

        if coin[index] {
            // generate range to get a random index
            // find the title of the minor from vector
            // clone the value
            // send it as a Some() value
            return Some(pool[rng.gen_range(0..pool.len())].clone())
        } else {
            return None
        }
    }
}