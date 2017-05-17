fn main() {
  run_all();
  //problem_11();
}
#[allow(dead_code)]

fn problem_11() -> i32 {
  let result = 0;

  println!("Problem 11: {:?}", result);
  result
}


fn problem_10() -> i64 {
  let result : i64 = Primes::new().take_while( |&x| x < 2_000_000 ).sum();

  result
}

// - - - - - - - - - - - - - - - - - - - -

fn problem_9() -> u64 {
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

  result
}

// - - - - - - - - - - - - - - - - - - - -

fn problem_8() -> u64 {
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

  result
}
// - - - - - - - - - - - - - - - - - - - -

fn problem_7() -> i64 {

  let result = Primes::new().nth(10_000).expect("Infinite, shouldn't fail to find");

  result
}

// - - - - - - - - - - - - - - - - - - - -

fn problem_6() -> u64 {
  let result = (1_u64..101_u64).fold(0,|acc,x| acc + x).pow(2) - (1_u64..101_u64).fold(0,|acc,x| acc + x.pow(2));

  result
}

// - - - - - - - - - - - - - - - - - - - -

fn problem_5() -> u64 {
  // Get unique prime factors, number has to be a multiple of the product of all of them,
  // so step by it
  let step : u64 = Primes::new().take_while( |&x| x < 21 ).product::<i64>() as u64;

  // let nums : Vec<_> = (2..21).filter( |&x| (x+1..21).filter(|&y| y % x == 0 ).collect::<Vec<_>>().len() == 0 ).rev().collect();

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

  result
}

// - - - - - - - - - - - - - - - - - - - -

fn problem_4() -> i64 {
  let mut result = 0;

  for i in 1..1000 {
    for j in i..1000 {
      let product = i * j;
      if number_is_palindrome(product) {
        result = std::cmp::max(result,product);
      }
    }
  }

  result
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

fn problem_3() -> i64 {
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

  result
}

struct Primes {
  test_next: i64,
  found_primes: Vec<i64>
}

impl Primes {
  fn new() -> Primes {
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

impl Iterator for Primes {
  type Item = i64;
  fn next(&mut self) -> Option<Self::Item> {
    if self.test_next == 2 {
      self.test_next = 3;
      return Some(2);
    }

    while !Primes::is_prime(&self.found_primes,self.test_next) {
      self.test_next += 2;
      if self.test_next > std::i64::MAX / 2 {
        println!("Max Fibonacci!");
        return None
      }
    }

    let latest = self.test_next;

    self.found_primes.push(latest);

    self.test_next += 2;

    Some(latest)
  }
}

// - - - - - - - - - - - - - - - - - - - -

fn problem_2() -> i32 {

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

  result
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

fn problem_1() -> i32 {
  let mut sum = 0;

  for i in 1..1000 {
    if i % 3 == 0 || i % 5 == 0 {
      sum += i;
    }
  }

  sum
}



#[allow(dead_code)]
fn run_all() {

  assert_eq!(problem_1(), 233168);
  assert_eq!(problem_2(), 4613732);
  assert_eq!(problem_3(), 6857);
  assert_eq!(problem_4(), 906609);
  assert_eq!(problem_5(), 232792560);
  assert_eq!(problem_6(), 25164150);
  assert_eq!(problem_7(), 104743);
  assert_eq!(problem_8(), 23514624000);
  assert_eq!(problem_9(), 31875000);
  assert_eq!(problem_10(), 142913828922);
  /*
  assert_eq!(problem_11(), 70600674);
  assert_eq!(problem_12(), 76576500);
  assert_eq!(problem_13(), 5537376230);
  assert_eq!(problem_14(), 837799);
  assert_eq!(problem_15(), 137846528820);
  assert_eq!(problem_16(), 1366);
  assert_eq!(problem_17(), 21124);
  assert_eq!(problem_18(), 1074);
  assert_eq!(problem_19(), 171);
  assert_eq!(problem_20(), 648);
  assert_eq!(problem_21(), 31626);
  assert_eq!(problem_22(), 871198282);
  assert_eq!(problem_23(), 4179871);
  assert_eq!(problem_24(), 2783915460);
  assert_eq!(problem_25(), 4782);
  assert_eq!(problem_26(), 983);
  assert_eq!(problem_27(), -59231);
  assert_eq!(problem_28(), 669171001);
  assert_eq!(problem_29(), 9183);
  assert_eq!(problem_30(), 443839);
  assert_eq!(problem_31(), 73682);
  assert_eq!(problem_32(), 45228);
  assert_eq!(problem_33(), 100);
  assert_eq!(problem_34(), 40730);
  assert_eq!(problem_35(), 55);
  assert_eq!(problem_36(), 872187);
  assert_eq!(problem_37(), 748317);
  assert_eq!(problem_38(), 932718654);
  assert_eq!(problem_39(), 840);
  assert_eq!(problem_40(), 210);
  assert_eq!(problem_41(), 7652413);
  assert_eq!(problem_42(), 162);
  assert_eq!(problem_43(), 16695334890);
  assert_eq!(problem_44(), 5482660);
  assert_eq!(problem_45(), 1533776805);
  assert_eq!(problem_46(), 5777);
  assert_eq!(problem_47(), 134043);
  assert_eq!(problem_48(), 9110846700);
  assert_eq!(problem_49(), 296962999629);
  assert_eq!(problem_50(), 997651);
  */

}
