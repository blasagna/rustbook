use std::collections::HashMap;

pub struct Company {
    name: String,
    departments: HashMap<String, Department>,
}

pub struct Department {
    name: String,
    employees: Vec<Employee>,
}

pub struct Employee {
    name: String,
    title: String,
}

impl Employee {
    pub fn new(name: &str, title: &str) -> Employee {
        Employee {
            name: name.to_string(),
            title: title.to_string(),
        }
    }
}

impl Department {
    pub fn new(name: &str) -> Department {
        Department {
            name: name.to_string(),
            employees: Vec::new(),
        }
    }

    pub fn add_employee(&mut self, empl: Employee) {
        self.employees.push(empl);
    }

    pub fn add_employees(&mut self, empls: Vec<Employee>) {
        for empl in empls {
            self.add_employee(empl);
        }
    }

    pub fn show(&self) {
        println!("department {}", self.name);
        for empl in &self.employees {
            println!(" - {} ({})", empl.name, empl.title); 
        }
    }
}

// TODO: add tests
impl Company {
  pub fn new(name: &str) -> Company {
    Company {
        name: name.to_string(),
        departments: HashMap::new()
    }
  }

  pub fn add_dept(&mut self, dept: Department) {
    self.departments.insert(dept.name.clone(), dept);
  }

  pub fn show_dept(&self, name: &str) {
    match self.departments.get(name) {
        None => println!("No employee records for department {}", name),
        Some(dept) => dept.show(),
    };
  }
  
  pub fn show_all(&self) {
    println!("Company {}", self.name);
    let mut n = 0;
    for (dept, _) in &self.departments {
      n += 1;
      self.show_dept(dept);
    }
    if n == 0 {
      println!("No department records for company {}", self.name);
    }
  }
}
