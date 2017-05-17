fn main() {
  problem_10();
}

#[allow(dead_code)]
fn problem_10() {
  let result = Primes::new().take_while( |&x| x < 2_000_000 ).sum::<i64>() + 2;

  println!("Problem 10: {:?}", result);
}

// - - - - - - - - - - - - - - - - - - - -

#[allow(dead_code)]
fn problem_9() {
  let mut result = 0;

  'outer: for c in 333_u64..997 {
    for b in 2..c {
      let a = 1000 - b - c;
      if a.pow(2) + b.pow(2) == c.pow(2) {
        result = a * b * c;
        break 'outer;
      }
    }
  }

  println!("Problem 9: {:?}", result);
}

// - - - - - - - - - - - - - - - - - - - -

#[allow(dead_code)]
fn problem_8() {
  let s = String::from("
    73167176531330624919225119674426574742355349194934
    96983520312774506326239578318016984801869478851843
    85861560789112949495459501737958331952853208805511
    12540698747158523863050715693290963295227443043557
    66896648950445244523161731856403098711121722383113
    62229893423380308135336276614282806444486645238749
    30358907296290491560440772390713810515859307960866
    70172427121883998797908792274921901699720888093776
    65727333001053367881220235421809751254540594752243
    52584907711670556013604839586446706324415722155397
    53697817977846174064955149290862569321978468622482
    83972241375657056057490261407972968652414535100474
    82166370484403199890008895243450658541227588666881
    16427171479924442928230863465674813919123162824586
    17866458359124566529476545682848912883142607690042
    24219022671055626321111109370544217506941658960408
    07198403850962455444362981230987879927244284909188
    84580156166097919133875499200524063689912560717606
    05886116467109405077541002256983155200055935729725
    71636269561882670428252483600823257530420752963450
    ").replace(" ","").replace("\n","");

  let mut result : u64 = 0;

  for i in 0..s.len()-12 {
    let substring = &s[i..i+13];
    let product : u64 = substring.chars().map( |c| c.to_digit(10).unwrap() as u64 ).product();
    result = std::cmp::max(result,product);
  }

  println!("Problem 8: {:?}", result);
}
// - - - - - - - - - - - - - - - - - - - -

#[allow(dead_code)]
fn problem_7() {

  let result = Primes::new().nth(9_999).expect("Infinite, shouldn't fail to find");

  println!("Problem 7: {:?}", result);
}

// - - - - - - - - - - - - - - - - - - - -

#[allow(dead_code)]
fn problem_6() {
  let result = (1_u64..101_u64).fold(0,|acc,x| acc + x).pow(2) - (1_u64..101_u64).fold(0,|acc,x| acc + x.pow(2));

  println!("Problem 6: {:?}", result);
}

// - - - - - - - - - - - - - - - - - - - -

#[allow(dead_code)]
fn problem_5() {
  // Get unique prime factors, number has to be a multiple of the product of all of them,
  // so step by it
  let step : u64 = Primes::new().take_while( |&x| x < 21 ).fold(2_u64, |acc,x| (x as u64) * acc );

  //let nums : Vec<_> = (2..21).filter( |&x| (x+1..21).filter(|&y| y % x == 0 ).collect::<Vec<_>>().len() == 0 ).rev().collect();

  // We can skip checking if a result is divisible by numbers that are divisors of the larger numbers
  // sorting by reverse should opt out of loop earlier
  let nums : Vec<u64> = (2..21).filter( |x| step % x != 0 ).rev().collect();
  let nums : Vec<u64> = nums.iter().cloned().filter( |x| nums.iter().filter(|&y| y != x && y % x == 0 ).count() == 0 ).collect();

  let mut result : u64 = 0;

  'outer: loop {
    result += step;
    for num in &nums {
      if result % num != 0 {
        continue 'outer;
      }
    }
    break;
  }

  println!("Problem 5: {:?}", result);
}

// - - - - - - - - - - - - - - - - - - - -

#[allow(dead_code)]
fn problem_4() {
  let mut result = 0;

  for i in 1..1000 {
    for j in i..1000 {
      let product = i * j;
      if number_is_palindrome(product) {
        result = std::cmp::max(result,product);
      }
    }
  }

  println!("Problem 4: {}", result);
}

fn digit_at(num:i64,index:u32) -> u32 {
  ((num / 10_i64.pow(index)) % 10) as u32
}

fn number_is_palindrome(num:i64) -> bool {
  let mut max_index = 0;
  let mut min_index = 0;
  while 10_i64.pow(max_index) <= num {
    max_index += 1;
  }
  max_index -= 1;

  while max_index > min_index {
    if digit_at(num,max_index) != digit_at(num,min_index) {
      return false;
    }
    max_index -= 1;
    min_index += 1;
  }

  true
}

// - - - - - - - - - - - - - - - - - - - -

#[allow(dead_code)]
fn problem_3() {
  let mut max : i64 = 600851475143;

  let primes = Primes::new();

  let mut result : i64 = 0;

  for prime in primes {
    while max % prime == 0 {
      result = std::cmp::max(result, prime);

      max /= prime;
    }

    if prime > max {
      break;
    }
  }

  println!("Problem 3: {}", result);
}

struct Primes {
  testing: i64,
  found_primes: Vec<i64>
}

impl Primes {
  fn new() -> Primes {
    Primes { testing: 1, found_primes: vec![] }
  }

  fn is_prime(found_primes : &Vec<i64>, testing : i64) -> bool {
    let max_prime = (testing as f64).sqrt() as i64;

    for &prime in found_primes {
      if prime > max_prime {
        break;
      }
      if testing % prime == 0 {
        return false;
      }
    }
    return true;
  }
}

impl Iterator for Primes {
  type Item = i64;
  fn next(&mut self) -> Option<Self::Item> {
    self.testing += 2;

    while !Primes::is_prime(&self.found_primes,self.testing) {
      self.testing += 2;
    }

    self.found_primes.push(self.testing);

    if self.testing > std::i64::MAX / 2 {
      println!("Max Fibonacci!");
      None
    } else {
      Some(self.testing)
    }
  }
}

// - - - - - - - - - - - - - - - - - - - -

#[allow(dead_code)]
fn problem_2() {

  let fib = Fibonacci::new();

  let mut result = 0;

  for num in fib {
    if num > 4_000_000 {
      break;
    }
    if num % 2 == 0 {
      result += num;
    }
  }

  println!("Problem 2: {}", result);
}

struct Fibonacci {
  previous: i32,
  current: i32
}

impl Fibonacci {
  fn new() -> Fibonacci {
    Fibonacci { previous: 0, current: 1 }
  }
}

impl Iterator for Fibonacci {
  type Item = i32;
  fn next(&mut self) -> Option<i32> {
    self.current += self.previous;
    self.previous = self.current - self.previous;
    if self.current > std::i32::MAX / 2 {
      println!("Max Fibonacci!");
      None
    } else {
      Some(self.current)
    }
  }
}

// - - - - - - - - - - - - - - - - - - - -

#[allow(dead_code)]
fn problem_1() {
  let mut sum = 0;

  for i in 1..1000 {
    if i % 3 == 0 || i % 5 == 0 {
      sum += i;
    }
  }

  println!("Problem 1: {}", sum);
}
