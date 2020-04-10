use std::collections::HashMap;

pub struct School {
    grades: HashMap<u32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            grades: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self.grades.contains_key(&grade) {
            let l = self.grades.get_mut(&grade).unwrap();
            l.push(student.to_string());
        } else {
            self.grades.insert(grade, vec![student.to_string()]);
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.grades.keys().cloned().collect::<Vec<_>>();

        grades.sort();

        grades
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.grades.get(&grade).map(|v| v.clone()).map(|mut v| {
            v.sort();
            v
        })
    }
}
