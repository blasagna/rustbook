mod directory;
mod stats;

fn main() {
  println!("{:?}", stats::summary_stats(vec![]));
  println!("{:?}", stats::summary_stats(vec![1, 2, 4, 3]));
  println!("{:?}", stats::summary_stats(vec![-1, 1]));
  println!("{:?}", stats::summary_stats(vec![-12, 1, 1]));


  let mut c = directory::Company::new("foo");
  c.show_all();
  c.show_dept("bar");
  c.add("alice", "bar");
  c.add("bob", "bar");
  c.add("chris", "baz");
  c.show_dept("bar");
  c.show_dept("baz");
  c.show_all();
}
