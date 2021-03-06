use std::io;

pub fn convert_to_bytes(codepoint: usize) -> String {
  find_octect(codepoint).expect("Octect not found").encode()
}

fn find_octect(codepoint: usize) -> Result<Octect, io::Error> {
  let converted_codepoint = format!("{:b}", codepoint);
  match converted_codepoint.len() {
    1..=7 => Ok(Octect::One { codepoint }),
    8..=11 => Ok(Octect::Two { codepoint }),
    12..=16 => Ok(Octect::Three { codepoint }),
    17..=21 => Ok(Octect::Four { codepoint }),
    _ => Err(io::Error::new(
      io::ErrorKind::InvalidData,
      "Unexpected or malformed UTF-8 input.",
    )),
  }
}

enum Octect {
  One { codepoint: usize },
  Two { codepoint: usize },
  Three { codepoint: usize },
  Four { codepoint: usize },
}

impl Octect {
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
  for bit in formatted_codepoint.chars().rev() {
    if tail.len() == 6 {
      tail = format!("10{}", tail);
      octect = format!("{}{}", tail, octect);
      tail = String::new();
    }
    tail = format!("{}{}", bit, tail);
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
      find_octect(0x61),
      Ok(Octect::One { codepoint: 0x61 })
    ));
    assert!(matches!(
      find_octect(0x0111),
      Ok(Octect::Two { codepoint: 0x0111 })
    ));
    assert!(matches!(
      find_octect(0x1EDF),
      Ok(Octect::Three { codepoint: 0x1EDF })
    ));
    assert!(matches!(
      find_octect(0x1F602),
      Ok(Octect::Four { codepoint: 0x1F602 })
    ));
    assert!(matches!(
      find_octect(0x7FFFFFFF),
      Err(_)
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
