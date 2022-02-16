pub fn convert_to_bytes(codepoint: usize) -> String {
  find_octect(&codepoint).encode()
}

fn find_octect(codepoint: &usize) -> Octect {
  let converted_codepoint = format!("{:b}", codepoint);
  match converted_codepoint.len() {
    1..=7 => Octect::One { codepoint },
    8..=11 => Octect::Two { codepoint },
    12..=16 => Octect::Three { codepoint },
    _ => Octect::Four { codepoint },
  }
}

enum Octect<'a> {
  One { codepoint: &'a usize },
  Two { codepoint: &'a usize },
  Three { codepoint: &'a usize },
  Four { codepoint: &'a usize },
}

impl<'a> Octect<'a> {
  fn encode(&self) -> String {
    match self {
      Octect::One { codepoint } => format!("0{:07b}", codepoint),
      Octect::Two { codepoint } => build_octect("110", format!("{:011b}", codepoint)),
      Octect::Three { codepoint } => build_octect("1110", format!("{:016b}", codepoint)),
      Octect::Four { codepoint } => build_octect("11110", format!("{:021b}", codepoint)),
    }
  }
}

fn build_octect(bit_prefix: &str, formatted_codepoint: String) -> String {
  let mut octect = String::new();
  let mut tail = String::new();
  for c in formatted_codepoint.chars().rev() {
    if tail.len() == 6 {
      tail = format!("10{}", tail);
      octect = format!("{}{}", tail, octect);
      tail = String::new();
    }
    tail = format!("{}{}", c, tail);
  }
  let head = format!("{}{}", bit_prefix, tail);
  format!("{}{}", head, octect)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_find_octect() {
    assert!(matches!(
      find_octect(&0x61),
      Octect::One { codepoint: 0x61 }
    ));
    assert!(matches!(
      find_octect(&0x0111),
      Octect::Two { codepoint: 0x0111 }
    ));
    assert!(matches!(
      find_octect(&0x1EDF),
      Octect::Three { codepoint: 0x1EDF }
    ));
    assert!(matches!(
      find_octect(&0x1F602),
      Octect::Four { codepoint: 0x1F602 }
    ));
  }

  #[test]
  fn test_build_octect() {
    let one_octect = convert_to_bytes(0x61);
    assert_eq!("01100001", one_octect);
    assert_eq!(8, one_octect.len());

    let two_octect = convert_to_bytes(0x0111);
    assert_eq!("1100010010010001", two_octect);
    assert_eq!(16, two_octect.len());

    let three_octect = convert_to_bytes(0x1EDF);
    assert_eq!("111000011011101110011111", three_octect);
    assert_eq!(24, three_octect.len());

    let four_octect = convert_to_bytes(0x1F602);
    assert_eq!("11110000100111111001100010000010", four_octect);
    assert_eq!(32, four_octect.len());
  }
}
