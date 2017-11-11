use std::collections::HashMap;

pub struct Company {
    name: String,
    departments: HashMap<String, Department>,
}

pub struct Department {
    name: String,
    employees: Vec<&Employee>,
}

pub struct Employee {
    name: String,
    title: String,
    manager: &Employee,
}

impl Employee {
    pub fn new(name: &str, title: &str) -> Employee {
        Employee {
            name: name.to_string(),
            title: title.to_string(),
        }
    }

    pub fn set_manager(&mut self, manager: &Employee) {
        self.manager = manager;
    }
}

impl Department {
    pub fn new(name: &str) -> Department {
        Department {
            name: name.to_string(),
            employees: Vec::new(),
        }
    }

    fn add_employee(&mut self, empl: &Employee) {
        self.employees.push(empl);
    }

    pub fn show(&self) {
        println!("department {}:", self.name);

    }
}

// TODO: add tests
impl Company {
  pub fn new(name: &str) -> Company {
    Company {
        name: name.to_string(),
        directory: HashMap::new()
    }
  }

  pub fn add(&mut self, empl: Employee, dept: &str) {
    let list = self.directory.entry(dept.to_string()).or_insert(vec![]);
    list.push(empl));
  }

  pub fn show_dept(&self, name: &str) {
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

