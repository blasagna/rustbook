use std::collections::HashMap;

pub struct Company {
name: String,
        directory: HashMap<String, Vec<String>>,
}

// TODO: add tests
impl Company {
  pub fn new(name: &str) -> Company {
    Company {
name: name.to_string(),
        directory: HashMap::new()
    }
  }

  pub fn add(&mut self, employee: &str, department: &str) {
    let list = self.directory.entry(department.to_string()).or_insert(vec![]);
    list.push(employee.to_string());
  }

  pub fn show_dept(&self, name: &str) {
    println!("department {}:", name);
    match self.directory.get(name) {
      None => println!(" - No employee records for department {}", name),
           Some(empls) => {
             for empl in empls {
               println!(" - {}", empl);
             }
           }
    };
  }

  pub fn show_all(&self) {
    let mut n = 0;
    println!("company {}:", self.name);
    for (dept, _) in &self.directory {
      n += 1;
      self.show_dept(dept);
    }
    if n == 0 {
      println!("No department records for company {}", self.name);
    }
  }
}

