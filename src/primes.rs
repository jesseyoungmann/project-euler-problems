use std;
use bit_vec::BitVec;

pub struct PrimesSieve {
  bits: BitVec,
  index: usize
}

impl PrimesSieve {
  pub fn new(max: usize) -> Self {
    let mut bits = BitVec::from_elem(max,true);
    bits.set(0,false);
    bits.set(1,false);
    Self {
      bits: bits,
      index: 2
    }
  }
}

// TODO: SKIP ALL 2's
impl Iterator for PrimesSieve {
  type Item = i64;
  fn next(&mut self) -> Option<Self::Item> {
    if self.index >= self.bits.len() {
      return None;
    }

    let result = self.index as i64;

    let mut i = self.index;
    i += self.index;
    while i < self.bits.len() {
      self.bits.set(i,false);
      i += self.index;
    }

    self.index += 1;
    while self.index < self.bits.len() && !self.bits.get(self.index).unwrap() {
      self.index += 1;
    }

    Some(result)
  }
}

pub struct Primes {
  test_next: i64,
  found_primes: Vec<i64>
}

// Doing prime.pow(2) instead of max_prime = sqrt doesn't seem to make a difference
impl Primes {
  pub fn new() -> Primes {
    Primes { test_next: 2, found_primes: vec![] }
  }

  fn is_prime(found_primes : &Vec<i64>, test_next : i64) -> bool {
    let max_prime = (test_next as f64).sqrt() as i64;

    for &prime in found_primes {
      if prime > max_prime {
        break;
      }
      if test_next % prime == 0 {
        return false;
      }
    }
    true
  }
}

const MAX_PRIME : i64 = std::i64::MAX / 2;

impl Iterator for Primes {
  type Item = i64;
  fn next(&mut self) -> Option<Self::Item> {
    if self.test_next == 2 {
      self.test_next = 3;
      return Some(2);
    }

    while !Primes::is_prime(&self.found_primes,self.test_next) {
      self.test_next += 2;
      if self.test_next > MAX_PRIME {
        println!("Max Prime!");
        return None
      }
    }

    let latest = self.test_next;

    self.found_primes.push(latest);

    self.test_next += 2;

    Some(latest)
  }
}
