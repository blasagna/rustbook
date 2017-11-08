use std::collections::HashMap;

pub struct Company {
    name: String,
    departments: HashMap<String, Department>,
}

pub struct Department {
    name: String,
    employees: Vec<String>,
}

//TODO: impl for employee, contractor, pointers needed?
//TODO: make department adds generic across employee/contractor/intern
pub struct Employee {
    name: String,
    ssn: String,
    title: String,
    location: String,
    age: u8,
    gender: String,
    manager: Employee,
    start_date: String,
}

pub struct Contractor {
    name: String,
    employer: Company,
    manager: Employee,
    title: String,
    location: String,
}

pub struct Intern {
    name: String,
    manager: Employee,
    title: String,
    location: String,
    end_date: String,
}

impl Department {
    pub fn new(name: &str) -> Department {
        Department {
            name: name.to_string(),
            employees: Vec::new(),
        }
    }

    pub fn add_employee(&mut self, name: &str) {
        self.employees.push(name);
    }

    pub fn add_employees(&mut self, names: Vec<&str>) {
        for name in names {
            self.add_employee(name);
        }
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

