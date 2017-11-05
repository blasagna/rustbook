use std::collections::HashMap;

#[derive(Debug)]
pub struct Stats {
mean: Option<f32>,
        median: Option<f32>,
        mode: Option<i32>
}

// TODO: add tests
pub fn summary_stats(numbers: Vec<i32>) -> Stats {
  let mut mean = None;
  let mut median = None;
  let mut mode = None;

  let mut sum: f32 = 0.0;
  let mut counts =  HashMap::new();

  // update sum for mean, counts for mode
  let mut sorted = numbers.clone();
  sorted.sort();
  for i in &sorted {
    sum += *i as f32;
    let count = counts.entry(i.to_string()).or_insert(0);
    *count += 1;
  }

  if sorted.len() > 0 {
    mean = Some(sum / (sorted.len() as f32));

    if sorted.len() % 2 != 0 {
      median = Some(sorted[sorted.len() / 2] as f32);
    } else {
      let upper = sorted[sorted.len() / 2] as f32;
      let lower = sorted[(sorted.len() / 2) - 1] as f32;
      median = Some((lower + upper) / 2.0)
    }
  }

  // determine mode
  let mut max_count = 0;
  for (num_string, count) in &counts {
    if mode == None || *count > max_count {
      mode = Some(*(&num_string.parse::<i32>().unwrap()));
      max_count = *count;
    }
  }

  Stats { mean, median, mode }
}
