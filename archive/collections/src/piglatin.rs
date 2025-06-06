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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn starts_with_consonant() {
        assert_eq!("oobar-fay", piggify("foobar"));
    }

    #[test]
    fn starts_with_vowel() {
        assert_eq!("oobar-hay", piggify("oobar"));
    }

    #[test]
    fn starts_with_y() {
        assert_eq!("aw-yay", piggify("yaw"));
    }

    #[test]
    fn empty() {
        assert_eq!("", piggify(""));
    }

    #[test]
    fn non_letters() {
        assert_eq!("2443.24--ay", piggify("-2443.24"));
    }
}
