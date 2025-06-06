mod directory;
mod stats;
mod piglatin;

fn main() {
  println!("{:?}", stats::summary_stats(vec![]));
  println!("{:?}", stats::summary_stats(vec![1, 2, 4, 3]));
  println!("{:?}", stats::summary_stats(vec![-1, 1]));
  println!("{:?}", stats::summary_stats(vec![-12, 1, 1]));
  println!();

  let mut c = directory::Company::new("foo");
  c.show_all();
  c.show_dept("bar");
  let mut sales = directory::Department::new("sales");
  let mut eng = directory::Department::new("engineering");
  let alice = directory::Employee::new("alice", "pos_a");
  let bob = directory::Employee::new("bob", "pos_b");
  sales.add_employees(vec![alice, bob]);
  let charlie = directory::Employee::new("charlie", "pos_c");
  eng.add_employee(charlie);
  c.add_dept(sales);
  c.add_dept(eng);
  c.show_dept("sales");
  c.show_dept("engineering");
  c.show_all();

  println!();

  println!("{}", piglatin::piggify("foobar"));
  println!("{}", piglatin::piggify("oobar"));
  println!("{}", piglatin::piggify(""));
  println!("{}", piglatin::piggify("-2443.24"));
}
