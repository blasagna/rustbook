// TODO: add tests
pub fn piggify(word: &str) -> String {
  let vowels = vec!['a', 'e', 'i', 'o', 'u'];
  let mut pigword = String::new();

  let mut first = None;
  for char in word.trim().chars() {
    if first == None {
      first = Some(char);
      if vowels.contains(&char) {
        pigword.push(char);
      }
      continue;
    }
    pigword.push(char)
  }

  if first != None {
    pigword.push('-');

    if vowels.contains(&first.unwrap()) {
      pigword.push('h');
    } else {
      pigword.push(first.unwrap());
    }
    pigword.push('a');
    pigword.push('y');
  }

  // new word is "start" + "-" + first letter + "ay"
  pigword
}
