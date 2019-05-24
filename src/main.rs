use std::{cmp, convert::TryInto, error::Error, u64};

use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "combomatic")]
struct Config {
  #[structopt(name = "MIN", long = "min", default_value = "0")]
  min: u64,
  #[structopt(name = "MAX", long = "max", default_value = "99")]
  max: u64,
  #[structopt(name = "RANGE", long = "range", default_value = "2")]
  range: u64,
  #[structopt(name = "COMBINATION", long = "combination")]
  combination: Vec<u64>,
  #[structopt(name = "CSV", long = "csv")]
  csv: bool,
}

fn modular_distance(a: u64, b: u64, modulus: u64) -> u64 {
  cmp::min((a + modulus - b) % modulus, (b + modulus - a) % modulus)
}

impl Config {
  fn guesses(&self) -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
    let base = self.range * 2 + 1;

    let numbers: u32 = self.combination.len().try_into()?;

    let last = base.pow(numbers);

    let mut guesses = Vec::new();

    for mut delta in 0..last {
      let mut guess = self.combination.clone();
      for n in &mut guess {
        let dn = delta % base;
        delta = delta / base;
        let offset = *n - self.min;
        *n = (offset + self.modulus() + dn - self.range) % self.modulus() + self.min;
      }
      guesses.push(guess);
    }

    guesses.sort_by_key(|guess| self.errors(guess));

    Ok(guesses)
  }

  fn modulus(&self) -> u64 {
    self.max - self.min + 1
  }

  fn errors(&self, guess: &[u64]) -> u64 {
    guess
      .iter()
      .zip(&self.combination)
      .map(|(g, c)| modular_distance(*g - self.min, *c - self.min, self.modulus()))
      .sum::<u64>()
  }

  fn run(self) -> Result<(), Box<dyn Error>> {
    let guesses = self.guesses()?;

    if self.csv {
      self.print_csv(&guesses);
    } else {
      self.print_unstructured(&guesses);
    }

    Ok(())
  }

  fn print_csv(&self, guesses: &[Vec<u64>]) {
    if guesses.is_empty() {
      return;
    }

    let numbers = guesses[0].len();

    print!("tried");
    for i in 0..numbers {
      print!(",number {}", i + 1);
    }
    print!(",errors");
    println!();

    for guess in guesses {
      for n in guess.iter() {
        print!(",{}", n);
      }
      print!(",{}", self.errors(guess));
      println!();
    }
  }

  fn print_unstructured(&self, guesses: &[Vec<u64>]) {
    let mut errors = u64::MAX;

    let width = self.max.to_string().chars().count();

    for guess in guesses {
      let guess_errors = self.errors(&guess);
      if guess_errors != errors {
        println!("{} errors:", guess_errors);
        errors = guess_errors;
      }

      for (i, n) in guess.iter().enumerate() {
        if i > 0 {
          print!("-");
        }
        print!("{:0width$}", n, width = width);
      }
      println!();
    }
  }
}

fn main() -> Result<(), Box<dyn Error>> {
  Config::from_args().run()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn range_zero() -> Result<(), Box<dyn Error>> {
    let combination = vec![0, 1, 2];

    let config = Config {
      min: 0,
      max: 99,
      range: 0,
      csv: false,
      combination: combination.clone(),
    };

    assert_eq!(config.guesses()?, &[combination]);

    Ok(())
  }

  #[test]
  fn range_one() -> Result<(), Box<dyn Error>> {
    let combination = vec![0];

    let config = Config {
      min: 0,
      max: 99,
      range: 1,
      csv: false,
      combination: combination.clone(),
    };

    assert_eq!(config.guesses()?.len(), 3);

    Ok(())
  }

  #[test]
  fn range_one_two() -> Result<(), Box<dyn Error>> {
    let combination = vec![0, 1];

    let config = Config {
      min: 0,
      max: 99,
      range: 1,
      csv: false,
      combination: combination.clone(),
    };

    assert_eq!(config.guesses()?.len(), 9);

    Ok(())
  }

  #[test]
  fn modular_distance_misc() {
    assert_eq!(modular_distance(0, 1, 10), 1);
    assert_eq!(modular_distance(0, 9, 10), 1);
    assert_eq!(modular_distance(0, 1, 2), 1);
    assert_eq!(modular_distance(0, 0, 2), 0);
    assert_eq!(modular_distance(0, 9, 11), 2);
    assert_eq!(modular_distance(0, 9, 100), 9);
  }
}
