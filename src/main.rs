#![allow(dead_code)]
extern crate num;

use num::bigint::BigUint;
use num::bigint::ToBigUint;
use num::ToPrimitive;

// Files
use std::fs::File;
use std::io::prelude::*;

fn main() {
  assert_eq!(problem_50(),ANSWERS[50]);
  //println!("Result: {:b}", 585);
  println!("Great work, team!");
}

// Using a vec of bools instead of binary search didn't make a diff, huh
fn problem_50() -> i64 {
  let primes : Vec<i64> = Primes::new().take_while( |&p| p < 1_000_000 ).collect();
  let max = primes[primes.len()-1];

  // (prime, len)
  // We know 41, with len 6, from problem specification
  let mut len = 0;
  let mut sum = 0;
  for &prime in &primes {
    sum += prime;
    if sum > max {
      sum -= prime;
      break;
    }
    len += 1;
  }

  let mut global_sum = sum;
  loop {
    let mut start = 0;
    sum = global_sum;
    //println!("sum: {}",sum);
    while sum <= max  {
      if let Ok(_) = primes.binary_search(&sum) {
        return sum;
      }

      sum += primes[start+len];
      sum -= primes[start];
      start += 1;
    }

    global_sum -= primes[len-1];
    len -= 1;
  }
}

fn old_problem_50() -> i64 {
  let primes : Vec<i64> = Primes::new().take_while( |&p| p < 1_000_000 ).collect();
  let max = primes[primes.len()-1];

  // (prime, len)
  // We know 41, with len 6, from problem specification
  let mut best : (i64, usize) = (41,6);
  'outer: for i in 0..primes.len()-1 {
    let mut sum = primes[i];
    let mut c = 1;
    for j in i+1..primes.len() {
      sum += primes[j];
      c += 1;
      if sum > max { continue 'outer; }
      if c <= best.1 { continue; }
      // Huh, binary_searching only the remaining subset of primes
      // is consistently slightly slower - overhead of creating a slice?
      if let Ok(_) = primes.binary_search(&sum) {
        if c > best.1 {
          best = (sum, c);
        }
      }
    }
  }
  best.0
}

fn problem_49() -> i64 {
  let primes : Vec<i64> = Primes::new().skip_while( |&p| p < 1_000 ).take_while( |&p| p < 10_000 ).collect();

  let mut reuse = vec!();

  // CAN ASSUME 'prime' IS THE SMALLEST, CAUSE WE'LL HIT IT FIRST
  // SO IF ANY PERMUTATIONS ARE PRIME BUT ARE SMALLER, BREAK
  'outer: for &prime in &primes {
    reuse.clear();
    for perm in Permuter::new(digits(prime)) {
      let p = from_digits_i8(&perm);
      if p == 1487 {
        continue 'outer;
      }
      if p == prime || p < 1000 {
        continue;
      }
      if let Ok(_) = primes.binary_search(&p) {
        reuse.push(p);
      }
    }

    // so we got some primes maybe
    // make them unique?
    reuse.sort_unstable();
    reuse.dedup();

    if reuse.len() < 2 {
      continue;
    }

    // SKIP THE LAST ONE
    for &p in &reuse[0..reuse.len()-1] {
      if let Ok(_) = reuse.binary_search(&(p+(p-prime))) {
        let mut output = vec!();
        for &p in [prime,p,p+(p-prime)].iter() {
          let mut digits = digits(p);
          digits.reverse();
          for d in digits {
            output.push(d);
          }
        }
        return from_digits_i8(&output);
      }
    }
  }
  unreachable!();
}

fn problem_48() -> i64 {
  fn chop_mul(a: u64, b: u64) -> u64 {
    let chop = 10_u64.pow(16);
    (a % chop) * (b %chop)
  }

  fn chop_add(a: u64, b: u64) -> u64 {
    let chop = 10_u64.pow(16);
    (a % chop) + (b %chop)
  }
  let mut result = 0;
  for i in 1..1001 {
    let mut val = i;
    for _ in 0..i-1 {
      val = chop_mul(val,i);
    }
    result = chop_add(result,val);
  }
  (result % 10_u64.pow(10)) as i64
}

// Find the first four consecutive integers to have four distinct prime factors each. What is the first of these numbers?
fn problem_47() -> i64 {
  // if a number has 4, start the count, if a number doesn't, reset the count to 0, if count == 4, break and return best

  let mut prime_g = Primes::new();
  let mut primes = vec!();
  primes.push(prime_g.next().unwrap());

  let mut best = (0,0);

  use std::collections::HashMap;
  // num => number_of_distinct_prime_factors
  type HelperMap = HashMap<i64,usize>;
  let mut memo : HelperMap = HashMap::new();
  memo.insert(1,0);

  let check = 4;

  for n in 2.. {
    while n > primes[primes.len()-1] {
      primes.push(prime_g.next().unwrap());
    }

    let mut distinct_pf = 0;
    if n == primes[primes.len()-1] {
      distinct_pf = 1;
      memo.insert(n,distinct_pf);
    } else {
      let mut x = n;
      for &prime in &primes {
        if x % prime == 0 {
          x /= prime;
          if let Some(&dpf) = memo.get(&x) {
            distinct_pf = dpf;
            if x % prime != 0 {
              distinct_pf += 1;
            }
            memo.insert(n,distinct_pf);
            break;
          } else {
            panic!("WTF MAN, IT SHOULD EXIST: {}",x);
          }
        }
      }
    }

    if distinct_pf == check {
      if best.0 == 0 {
        best = (n,1);
      } else {
        best.1 += 1;
      }
    } else {
      best = (0,0);
    }

    if best.1 == check {
      return best.0;
    }
  }
  unreachable!();
}


fn problem_46() -> i64 {
  // for each odd composite number (iterate by 2, start at 3), throw away if it is prime
  // if not,
  // for each prime smaller than it, subtract that prime
  // CHECK IF IT IS ODD, OTHERWISE TOSSING
  let mut prime_g = Primes::new();
  let mut primes = vec!();
  prime_g.next(); // throw away 2
  primes.push(prime_g.next().unwrap());

  'outer: for n in 1.. {
    let n = n * 2 + 1;

    while n > primes[primes.len()-1] {
      primes.push(prime_g.next().unwrap());
    }

    if let Ok(_) = primes.binary_search(&n) {
      continue;
    }

    for &prime in &primes {
      if prime >= n { break; }

      let mut x = n - prime;
      if x % 2 != 0 { continue; }
      x /= 2;
      if ((x as f64).sqrt() as i64).pow(2) == x {
        continue 'outer;
      }
    }
    return n;
  }
  unreachable!();
}

fn problem_45() -> i64 {
  fn triangle(n: i64) -> i64 { n * (n + 1) / 2 }
  fn pentagonal(n: i64) -> i64 { n * (3 * n - 1) / 2 }
  fn hexagonal(n: i64) -> i64 { n * (2 * n - 1) }

  let mut i = 2;
  let mut j = 2;

  for k in 2.. {
    let p_k = hexagonal(k);
    while triangle(i) < p_k {
      i += 1;
    }
    while pentagonal(j) < p_k {
      j += 1;
    }
    if p_k != 40755 && p_k == triangle(i) && p_k == pentagonal(j) {
      return p_k
    }
  }
  unreachable!();
}

fn problem_44() -> i64 {
  fn add_pentagonals(ps: &mut Vec<i64>, starting: usize, max: i64) -> usize {
    for i in starting.. {
      let next = (i.checked_mul(3 * i - 1).unwrap() / 2) as i64;
      ps.push(next);
      if next >= max {
        return i+1;
      }
    }
    unreachable!();
  }

  let mut pentagonals = vec!();
  let mut starting = 1;
  starting = add_pentagonals(&mut pentagonals,starting,100);

  let mut gap = 1;

  for x in 0.. {
    for i in 0..x {
      let p_j = pentagonals[i];
      let p_k = pentagonals[i + gap];
      let sum = p_j + p_k;
      if sum > pentagonals[pentagonals.len()-1] {
        starting = add_pentagonals(&mut pentagonals, starting, sum);
      }

      if let Err(_) = pentagonals.binary_search(&(p_k - p_j)) {
        continue;
      }

      if let Err(_) = pentagonals.binary_search(&sum) {
        continue;
      }

      return p_k - p_j;
    }
    gap += 1;
  }
  unreachable!();
}

//slower cause of starting at 0
//if we fail on the %2 test, we can add directly to permuter index
//to skip all the smaller ones (until we would flip the ..4 place)
fn problem_43() -> i64 {
  let mut result = 0;
  let mut perm = vec!();
  let mut permuter = Permuter::new(vec![0,1,2,3,4,5,6,7,8,9]);
  // IF IT LOOPS AROUND, IT FAILS
  while permuter.permute(&mut perm) {
    if from_digits_i8(&perm[1..4]) % 2 != 0 {
      permuter.index += 6 * 5 * 4 * 3 * 2 - 1;
      continue;
    }
    if from_digits_i8(&perm[2..5]) % 3 != 0 {
      permuter.index += 5 * 4 * 3 * 2 - 1;
      continue;
    }
    if from_digits_i8(&perm[3..6]) % 5 != 0 {
      permuter.index += 4 * 3 * 2 - 1;
      continue;
    }
    if from_digits_i8(&perm[4..7]) % 7 != 0 {
      permuter.index += 3 * 2 - 1;
      continue;
    }
    if from_digits_i8(&perm[5..8]) % 11 == 0 &&
      from_digits_i8(&perm[6..9]) % 13 == 0 &&
      from_digits_i8(&perm[7..10]) % 17 == 0 {
      result += from_digits_i8(&perm);
    }
  }
  result
}

fn problem_42() -> i64 {
  let mut triangle_numbers = vec!();
  for n in 1..10_000 {
    triangle_numbers.push( (n * (n+1) / 2) );
  }
  let triangle_word = |word: &str| {
    let mut sum = 0;
    for c in word.chars() {
      assert!(c != '"');
      sum += c as usize - 'A' as usize + 1;
    }
    triangle_numbers.binary_search(&sum).is_ok()
  };

  //println!("triangle_word? {}",triangle_word(&"SKY"));

  let mut f = File::open("assets/p042_words.txt").expect("File not found!");
  let mut file_text = String::new();
  f.read_to_string(&mut file_text).expect("Something went wrong reading file!");

  let mut result = 0;
  for word in file_text.split(",") {
    let word = &word[1..word.len()-1]; //chop off quotes
    if triangle_word(&word) {
      result += 1;
    }
  }
  result
}

// just loading primes is slow
// instead, let's use permutations
// starting with all 9
// and break if we find a prime
fn problem_41() -> i64 {
  fn is_prime(num: i64) -> bool {
    let max = (num as f64).sqrt() as i64;

    for i in 2..max+1 {
      if num % i == 0 {
        return false;
      }
    }
    true
  }

  for i in 4..9 {
    let mut digits = (1..(13-i)).collect::<Vec<_>>();
    // Reverse so we start with the largest?
    digits.reverse();
    for perm in Permuter::new(digits) {
      let possible = from_digits(&perm);
      if is_prime(possible) {
        return possible;
      }
    }
  }

  0
}

fn problem_40() -> i64 {
  let indexes = [1,10,100,1_000,10_000,100_000,1_000_000];
  let mut i = 0;
  let mut index = indexes[i];
  let mut result : i64 = 1;

  let mut reuse = vec!();
  let mut so_far = 0;

  for num in 1.. {
    digits_reuse(num,&mut reuse);
    so_far += reuse.len();
    if index > so_far {
      //index -= reuse.len();
      //println!("not big enough: {},{}",index,so_far);
    } else {
      //println!("big enough: {},{},{:?}",index,so_far,reuse);
      // cause it's reversed, do so_far - index. Some -1's cancel out from reversing
      // and from indexes not being 0 based
      result *= reuse[so_far - index] as i64;

      i += 1;
      if i == indexes.len() { break; }
      index = indexes[i]
    }
  }
  result
}

fn problem_39() -> i64 {
  let mut best = (0,0);
  for perim in 10..1001_i64 {
    let mut count = 0;
    // how to limit hyp, can't be bigger than when
    // hyp can get close to half, imagine 1 being one edge
    // but at smallest it must have equal other edges?

    // TODO: Find a better lower bound
    for hyp in 1..perim/2 {
      // Only have to check up to half of the remaining space
      for edge1 in 1..(perim-hyp+1)/2 {
        let edge2 = perim - hyp - edge1;
        //println!("{},{},{}",hyp,edge1,edge2);
        if hyp.pow(2) == edge1.pow(2) + edge2.pow(2) {
          count += 1;
        }
      }
    }
    if count > best.1 {
      best = (perim,count);
    }
  }
  best.0
}

fn problem_38() -> i64 {

  let mut results = vec!();
  let mut digits : Vec<i8> = vec!();

  'outer: for num in 10..10_000 {
    let mut cache = [false; 10];
    // Can't use 0, so mark it as already used
    cache[0] = true;
    let mut count = 0;

    // can't be i of 1 only, right? cause the max is too small
    let mut pandigital = 0_i64;
    for i in 1.. {
      let prod = num * i;
      digits_reuse(prod, &mut digits);
      digits.reverse();
      for &d in &digits {
        if cache[d as usize] { continue 'outer; }
        cache[d as usize] = true;
        pandigital = pandigital * 10 + d as i64;
        count += 1;
      }

      if count == 9 { break; }
      if count > 9 { continue 'outer; }
    }
    results.push(pandigital);
  }

  results.sort_unstable();
  *results.last().unwrap_or(&0)
}

fn largest_power_of_ten(num: i64) -> u32 {
  let mut pow = 0;
  while 10_i64.pow(pow) <= num {
    pow += 1;
  }
  pow -= 1;
  pow
}

fn digit_at_pow(num: i64, pow: u32) -> i64 {
  ((num / 10_i64.pow(pow)) % 10) * 10_i64.pow(pow)
}

fn problem_37() -> i64 {
  let mut result = vec!();

  let mut owned_primes = vec!();
  //let owned_primes = Primes::new().take_while(|&p| p < 1_000_000).collect::<Vec<i64>>();

  'outer: for prime in Primes::new() {
  //'outer: for prime in 3797..3798 {
    if prime > 1_000_000 {
      println!("WTF");
      break;
    }
    owned_primes.push(prime);
    {
      let mut prime = prime;
      let mut pow = largest_power_of_ten(prime);
      loop {
        prime -= digit_at_pow(prime,pow);
        //println!("{}",prime);
        if let Err(_) = owned_primes.binary_search(&prime) {
          continue 'outer;
        }
        pow -= 1;
        if pow == 0 { break; }
      }
    }
    {
      let mut prime = prime;
      prime = prime / 10;
      while prime > 0 {
        //println!("{}",prime);
        if let Err(_) = owned_primes.binary_search(&prime) {
          continue 'outer;
        }
        prime = prime / 10;
      }
    }
    result.push(prime);
    if result.len() == 11 {
      break;
    }
  }

  result.iter().sum()
}

fn number_is_binary_palindrome(num:i64) -> bool {
  fn digit_at(num:i64,index:u32) -> i64 {
    (num >> index) % 2
  }

  let mut max_index = 0;
  let mut min_index = 0;
  while 2_i64.pow(max_index) <= num {
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

fn problem_36() -> i64 {
  let mut result = 0;
  for n in 1..1_000_000 {
    if number_is_palindrome(n) && number_is_binary_palindrome(n) {
      result += n;
    }
  }
  result
}

fn problem_35() -> i64 {
  let owned_primes = Primes::new().take_while(|&p| p < 1_000_000).collect::<Vec<i64>>();

  let mut result = 1; // seed with 1 for `2`, which would break our prime testing
  let mut digits : Vec<i8> = vec!();

  'outer: for i in 3..1_000_000 {
    if i % 2 == 0 { continue; }
    digits_reuse(i, &mut digits);
    if digits.iter().any(|d| d % 2 == 0) { continue; }

    let len = digits.len();
    for i in 0..len {
      if let Err(_) = owned_primes.binary_search(&from_digits_i8(&digits)) {
        continue 'outer;
      }
      if i != len-1 {
        let first = digits[0];
        for j in 1..len {
          digits[j-1] = digits[j];
        }
        digits[len-1] = first;
      }
    }

    result += 1;
  }
  result
}

fn factorial(n: i64) -> i64 {
  (1..n+1).fold(1, |p, n| p * n )
}

fn digits_reuse(num: i64, digits: &mut Vec<i8> ) {
  digits.clear();
  let mut num = num;
  if num == 0 {
    digits.push(0);
    return;
  }

  while num > 0 {
    let rem = num % 10;
    let quot = num / 10;
    digits.push(rem as i8);
    num = quot;
  }
}

fn problem_34() -> i64 {
  let mut factorials = [0;10];
  for i in 0..10 {
    factorials[i] = factorial(i as i64);
  }

  // WORKS, BUT 9 SECONDS ON RELEASE VERSION
  // Got it down by re-using digits, like 1.9 seconds
  // still slowest by far
  let mut max = 10;
  let mut decimal = 1;
  while max < factorial(9) * decimal {
    max *= 10;
    decimal += 1;
  }
  //println!("MAX: {}",max);

  let mut result = 0;
  let mut digits : Vec<i8> = vec!();
  for i in 10..max {
    // COULD BE SLICE
    digits_reuse(i,&mut digits);
    if digits.iter().map(|i| factorials[*i as usize] ).sum::<i64>() == i {
      result += i;
    }
  }
  result
}


// digit canceling fractions
//There are exactly four non-trivial examples of this type of fraction, less than one in value, and containing two digits in the numerator and denominator.

// If the product of these four fractions is given in its lowest common terms, find the value of the denominator.
fn problem_33() -> i64 {
  fn lowest_common_terms(frac: (i64,i64) ) -> (i64,i64) {
    let (mut numerator, mut denominator) = frac;
    for i in 2..std::cmp::min(numerator, denominator)+1 {
      while numerator % i == 0 && denominator % i == 0 {
        numerator /= i;
        denominator /= i;
      }
    }
    (numerator, denominator)
  }

  //println!("!!!{:?}",lowest_common_terms((10,160)));
  //just do float comparison and use an epsilon?
  let mut results = vec!();
  for n in 1..10 {
    for x in 1..10 {
      for y in 1..10 {
        if x == n || y == n { continue; }
        // type one
        let t1_n = from_digits(&[x,n]);
        let t1_d = from_digits(&[n,y]);

        // COULD REPLACE THIS WITH JUST PUTTING THE SECOND IN LOWEST COMMON TERMS, BUT
        // THIS IS MUCH FASTER
        if ((t1_n as f64) / (t1_d as f64) - (x as f64) / (y as f64)).abs() < 0.0000000000001 {
          results.push((t1_n,t1_d));
        }
      }
    }
  }

  lowest_common_terms(
    results.iter().fold((1,1), |(a,b), &(n,d)| lowest_common_terms((a * n, b * d)) )
  ).1
}

struct Permuter<T : Clone> {
  items: Vec<T>,
  index_used: Vec<bool>,
  current_indexes: Vec<usize>,
  index: usize
}

impl<T : Clone> Permuter<T> {
  fn new(items: Vec<T>) -> Self {
    Self {
      items: items,
      index: 0,
      index_used: vec!(),
      current_indexes: vec!()
    }
  }
}

/* HOKAY, SO: we just have one index, that we increment by one each permutation. To map an
 * index to a unique permutation, we use a 'factorial mod' math, like grabbing digits, but
 * each time you div_mod, you decrease the denominator by one, because the sub array you
 * need to grab from is smaller after the previous extraction.
 * Then, to avoid creating actual sub arrays to index into with that list of indexes,
 * we do a tricky iterating index, where you skip over already used indexes, and otherwise
 * decrement the index until you find your item in the original item list.
 *
 * Okay, I updated it to increase the denominator by one in facmod, so we get lexicographic
 * permutation order, and then we reverse the index order
 * Because we want the first digit to vary the slowest, but have the most options, so we do
 * index % len LAST.
 */
impl<T : Clone> Iterator for Permuter<T> {
  type Item = Vec<T>;
  fn next(&mut self) -> Option<Vec<T>> {
    let mut result : Vec<T> = vec!();
    // moves result inside, returns it
    if self.permute(&mut result) {
      Some(result)
    } else {
      None
    }
  }
}

impl<T : Clone> Permuter<T> {
  fn permute(&mut self, result: &mut Vec<T>) -> bool {
    // TODO CHECK COST IF 0 LEN
    result.clear();
    // Quit if we hit the item length factorial, which is the total number of
    // permutations
    // We could also check if the factorial-mod index list is all 0's again, but
    // then we'd have to track a 'seen 0 once' bool
    if self.index >= (1..self.items.len()+1).fold(1, |p, n| p*n) {
      return false;
    }

    let len = self.items.len();
    self.index_used.clear();
    for _ in 0..len {
      self.index_used.push(false);
    }
    self.current_indexes.clear();
    let mut index = self.index;
    let mut modn = 1;
    while modn <= len {
      let j = index % modn;
      index = index / modn;
      modn += 1;
      self.current_indexes.push(j);
    }

    self.current_indexes.reverse();

    for &index in &self.current_indexes {
      let mut index = index;
      let mut i = 0;
      loop {
        if self.index_used[i] {
          i += 1;
        } else if index == 0 {
          break;
        } else {
          index -= 1;
          i += 1;
        }
      }
      result.push(self.items[i].clone());
      self.index_used[i] = true;
    }

    self.index += 1;
    true
  }
}

fn from_digits_i8(digits: &[i8]) -> i64 {
  let mut result : i64 = 0;
  for n in digits {
    result = result * 10 + (*n as i64);
  }
  result
}

fn from_digits(digits: &[i32]) -> i64 {
  let mut result : i64 = 0;
  for n in digits {
    result = result * 10 + (*n as i64);
  }
  result
}

fn problem_32() -> i64 {
  let digits : Vec<i32> = (1..10).collect();
  let mut results = vec!();
  let len = digits.len();

  for perm in Permuter::new(digits) {
    for split_a in 1..len-2 {
      for split_b in split_a+1..len-split_a-1 {
        let a = from_digits(&perm[0..split_a]);
        let b = from_digits(&perm[split_a..split_b]);
        let c = from_digits(&perm[split_b..len]);
        if a * b == c {
          results.push(c);
        }
      }
    }
  }

  results.sort_unstable();
  results.dedup();
  let sum : i64 = results.iter().sum();
  sum as i64
}

fn problem_31() -> i64 {
  use std::collections::HashMap;
  // (amount, coins.len()) => number of ways
  type HelperMap = HashMap<(i64,i64),i64>;
  let mut memo : HelperMap = HashMap::new();

  let mut coins = vec!(1,2,5,10,20,50,100,200);
  coins.reverse();

  fn helper(amount: i64, coins: &[i64], memo: &mut HelperMap) -> i64 {
    assert!(amount > 0);

    let len = coins.len() as i64;

    if len == 0 { return 0; }

    if let Some(&ways) = memo.get(&(amount,len)) { return ways; }

    let mut ways = 0;
    for i in 0..coins.len() {
      let coin = coins[i];
      let next_amount = amount - coin;

      if next_amount == 0 {
        ways += 1;
      } else if next_amount > 0 {
        ways += helper(next_amount, &coins[i..(len as usize)], memo);
      }
    }

    memo.insert((amount,len),ways);
    ways
  }

  helper(200,&coins,&mut memo)
}

fn problem_30() -> i64 {
  fn sum_of_power_of_digits(n: i64, pow: u32) -> i64 {
    digits(n).into_iter().map( |i| (i as i64).pow(pow) ).sum::<i64>()
  }
  let mut result = 0;
  // CHECK PREVIOUS COMMIT FOR CLEANER VERSION THATS JUST AS FAST
  //
  // OKAY, WHAT'S THE INTUITION
  // SIMPLEST IS 1 mil max, since 999_999 > (9 ** 5) * 6
  // but 99_999 < (9 ** 5) * 5
  // in ruby I calculated more, by decreasing from 1 mil, starting with the most sig digit,
  // until it was smaller than the sum again (with max 9s below?)
  // this runs in under a second, but what can I get it down to?

  // codify process

  let mut maxest_max = 99;
  let mut maxest_decimal = 10;
  while maxest_max < sum_of_power_of_digits(maxest_max, 5) {
    maxest_max = maxest_max * 10 + 9;
    maxest_decimal *= 10;
  }

  // SHAVED A TINY BIT OF TIME OFF, 0.8X to 0.79
  // gotta be something better?
  // oh right, we chose 1 mil cause 99_999 WASN'T bigger, so anything smaller won't matter
  // gotta revert
  // I could run this for every 1xxxx, 2xxxx, 3xxxx - recursively do it whenever we have
  // enough while counting down that we know we can skip all below ones
  // 99_999 is smaller, but 19_999 probably isn't. so whenever we hit a new turning point,
  // we could start winding down again
  // but how to know when?
  while maxest_max > 10 {
    let mut max = maxest_max;
    let mut decimal = maxest_decimal;
    loop {
      if decimal < 10 { break; }
      let next_max = max - decimal;
      let x = sum_of_power_of_digits(next_max, 5);
      if next_max == x {
        max = next_max;
        break;
      }
      if next_max > x {
        max = next_max;
      } else {
        decimal = decimal / 10;
      }
    }

    for n in maxest_decimal..(max + 1) {
      if sum_of_power_of_digits(n,5) == n {
        result += n;
      }
    }
    maxest_max /= 10;
    maxest_decimal /= 10;
  }
  result
}

fn digits(num: i64) -> Vec<i8> {
  let mut num = num;
  if num == 0 {
    return vec![0]
  }

  let mut digits : Vec<i8> = Vec::new();

  while num > 0 {
    let rem = num % 10;
    let quot = num / 10;
    digits.push(rem as i8);
    num = quot;
  }

  digits
}

/*
struct Digiterator {
  num: i64
}

fn digits_iter(num: i64) -> Digiterator {
  Digiterator {
    num: num
  }
}

impl Iterator for Digiterator {
  type Item = i8;
  fn next(&mut self) -> Option<Item> {
    if num <= 0 {
      return None;
    }

    let rem = self.num % 10;
    self.num /= 10;
    Some(rem as i8)
  }
}
*/

// FUCK THIS: THE GOTCHA IS THAT MANY OF THE RESULTS ARE TOO LARGE
// WORKS FINE IN RUBY CAUSE THEY'RE BIGINTS BY DEFAULT
fn problem_29() -> i64 {
  let mut results = vec!();
  for a in 2_i64..101 {
    for b in 2..101 {
      results.push(num::pow::pow(a.to_biguint().unwrap(),b));
    }
  }
  results.sort_unstable();
  results.dedup();
  results.len() as i64
}

// Diagonal sum
fn problem_28() -> i64 {
  let mut edge_size = 2;
  let mut sum = 1;
  let mut n = 1;
  while edge_size <= 1000 {
    for _ in 0..4 {
      n += edge_size;
      sum += n;
    }
    edge_size += 2;
  }
  sum
}

// Quadratic Primes
fn problem_27() -> i64 {
  let mut latest_best = (0,0,0);
  let mut primes = Primes::new();
  let mut owned_primes = vec!(primes.next().unwrap());

  for a in -999..1000 {
    for b in -1000..1001 {

      let mut count = 0;
      for n in 0_i64.. {
        let x = n.pow(2) + a * n + b;

        if x > owned_primes[owned_primes.len()-1] {
          loop {
            let next = primes.next().unwrap();
            owned_primes.push(next);
            if x <= next { break; }
          }
        }

        match owned_primes.binary_search(&x) {
          Ok(_) => (),
          Err(_) => break
        }

        count +=1;
      }

      if count > latest_best.2 {
        latest_best = (a,b,count);
      }
    }
  }
  latest_best.0 * latest_best.1
}


// longest recurring cycle
// okay, use ints, do long division with div and rem
// keep a list of possible cycling?
// push into array, check if it matches first
// what if you start with the same one? don't check for match unless you've had at least 2
// distinct chars?
// if you only ever see one char, at then end (max checks) just decide it was repeating 1
// this counts for 0 as well
//
// how to handle 0.1(6), where the first never re-occurs?
// will the longest pattern possibly have a non repeating start?
fn problem_26() -> i64 {
  let mut pattern = vec!();

  fn advance_long_division(num: i64,rem: i64) -> (i64,i64) {
    let times = rem / num;
    let o = (rem % num) * 10;
    (times, o) //dropping down 0, cause 1 is like that
  }

  fn verify_pattern(pattern: &[i64], num: i64, out: i64, rem: i64) -> bool {
    let mut out = out;
    let mut rem = rem;
    // TODO: not as many checks for longer patterns?
    for i in 0..(pattern.len() * 10) {
      if pattern[i % pattern.len()] != out {
        return false;
      }
      let (_out,_rem) = advance_long_division(num,rem);
      out = _out;
      rem = _rem;
    }
    true
  }


  let mut best = (1,1);

  'outer: for num in 2..1000 {
  //'outer: for num in 13..14 {
    pattern.clear();
    let mut num = num;
    let mut rem = 1;

    // Skip any pre-pattern shit
    // 20 just happens to work (I think actually 10 is the longest
    // starting amount, any way to prove that?
    for i in 0..20 {
      let (out,_rem) = advance_long_division(num,rem);
      rem = _rem;
      if i == 19 {
        pattern.push(out);
      }
    }

    assert!(pattern.len() != 0);

    // Eventually there will be a pattern!
    loop {
      let (out,_rem) = advance_long_division(num,rem);

      if verify_pattern(&pattern,num,out,_rem) {
        if pattern.len() > best.1 {
          best = (num,pattern.len());
        }
        continue 'outer;
      } else {
        rem = _rem;
        pattern.push(out);
      }
    }
  }
  best.0 as i64
}

// Skip using BigUint by recording chopping off lowest digit
// and recording number of chopped digits
fn problem_25() -> i64 {
  let mut i = 2;
  let mut a = 1_u64;
  let mut b = 1_u64;
  let mut digits = 0_u64;
  loop {
    let mut current_digits = digits;
    let mut current = b;
    while current > 0 {
      current /= 10;
      current_digits += 1;
    }

    if current_digits >= 1000 {
      break;
    }

    b += a;
    a = b - a;

    if b > std::u64::MAX / 2 {
      a /= 10;
      b /= 10;
      digits += 1;
    }

    i += 1;
  }
  i
}

#[allow(dead_code)]
fn problem_24() -> i64 {
  //from_digits(&Permuter::new(vec![0,1,2,3,4,5,6,7,8,9]).nth(1_000_000-1).unwrap()) as i64
  // Okay, so! Since Permuter just increments an index after each, just fuck with the index
  // directly!
  // TODO: Implement this as a `skip` method on the iterator
  let mut p = Permuter::new(vec![0,1,2,3,4,5,6,7,8,9]);
  p.index = 1_000_000-1;
  from_digits(&p.next().unwrap()) as i64
  //(0..10).permutation().get(1_000_000).expect()
}

/*
struct Permutation<T : Clone> {
  items: Vec<T>,
  index: u64
}

impl<T : Clone> Permutation<T> {
  fn new(items : Vec<T>) -> Permutation<T> {
    Permutation { items: items, index: 0 }
  }

  fn max(&self) -> u64 {
    (1_u64..(self.items.len() as u64 + 1)).product()
  }
}

impl<'a,T> Iterator for Permutation<T> {
  type Item = Vec<&'a T>;
  fn next(&'a mut self) -> Option<Self::Item> {
    let v = self.items.iter().collect();
    Some(v)
  }
}
*/

// Debug is 12 seconds vs 1.7 for release, wonder what's going on here?
// looks like using binary search instead of a hashmap is not faster,
// 16 seconds instead of 12
// very similar time for release? or losing back some optimized time?
fn problem_23() -> i64 {
  let limit = 28123_usize + 1;
  let mut abundant_numbers : Vec<usize> = Vec::new();
  let mut is_abundant : std::collections::HashMap<usize,bool> = std::collections::HashMap::new();

  let primes = Primes::new().take_while(|&p| p < limit as i64).collect::<Vec<i64>>();
  let mut reuse = vec!();

  for i in 12_usize..limit {
    // i+i since properest returns the value itself
    // TODO: Memoize? Memoize just with the sum?
    properest_divisors(i as i64,&primes,&mut reuse);
    if reuse.iter().sum::<i64>() as usize > i+i {
      abundant_numbers.push(i);
      is_abundant.insert(i,true);
    }
  }

  // We know all up to 24 are not sums of two abundant numbers
  let mut result : usize = (1..24).sum();

  'outer: for num in 25..limit {
    for &abundant in &abundant_numbers {
      // Only check if smaller, then break and add num
      if abundant > (num / 2 + 1) {
        break;
      }

      if let Some(_) = is_abundant.get(&(num - abundant)) {
        continue 'outer;
      }
    }
    result += num;
  }

  result as i64
}

#[allow(dead_code)]
fn proper_divisors(num : usize) -> Vec<usize> {
  let mut result = vec!(1);
  for i in 2..(num / 2 + ( 1 - num % 2 )) {
    if num % i == 0 {
      result.push(i);
    }
  }
  result
}

fn sum_of_proper_divisors(num: usize) -> usize {
  let mut sum = 1;
  for i in 2..(num / 2 + (1 - num % 2)) {
    if num % i == 0 {
      sum += i;
    }
  }
  sum
}

fn new_sum_of_proper_divisors(num: usize, divisor_tree: &Vec<Vec<usize>>) -> usize {
  let mut sum = 1;
  let max = num / 2 + (1 - num % 2);
  for v in divisor_tree {
    if v[0] > max { break; }
    for (i,&d) in v.iter().enumerate() {
      if d > max { break; }
      if num % d == 0 {
        sum += d;
      } else {
        // If the multiple is not a divisor, no future ones should be?
        // maybe just, if first is not, break, else check the rest
        if i == 0 {
          break;
        }
      }
    }
  }
  sum
}

// hmm, could we build a tree of divisors?
// loop through each, multiples of each prime that are unique to that prime
// TODO: use std::collections::BitVec;
fn divisor_tree(max: usize) -> Vec<Vec<usize>> {
  use std::collections::HashMap;
  let mut result = vec!();
  let mut seen : HashMap<usize,bool> = HashMap::new();
  for prime in Primes::new() {
    let prime = prime as usize;
    if prime > max { break; }
    let mut stuff = vec!(prime);
    seen.insert(prime,true);

    let mut multiple = prime + prime;
    while multiple < max {
      let was_seen = seen.get(&multiple).is_some();
      if !was_seen {
        seen.insert(multiple,true);
        stuff.push(multiple);
      }
      multiple += prime;
    }

    result.push(stuff);
  }
  result
}


//fn properest_divisors(num: i64, primes: &Vec<i64>, reuse: &mut Vec<i64>) -> i64 {
fn properest_divisors(num: i64, primes: &Vec<i64>, results: &mut Vec<i64>) {
  results.clear();
  //let mut results = reuse;
  //let mut results = vec!(1);
  results.push(1);
  let mut num = num;

  for &prime in primes {
    if num == 1 { break; }

    // HOW TO HANDLE PRIME POWERS? ONLY START AT THE NEW ONES?
    // anything you've already multiplied 2 by is used
    let mut starting = 0;
    let len = results.len();

    while num % prime == 0 {
      num = num / prime;

      for i in starting..len+starting {
        let o = results[i] * prime;
        results.push(o);
      }
      starting += len;
      //println!("prime: {}, num: {}, results: {:?}",prime,num,results);
    }
  }
  //results.sort_unstable();
  //results
}

fn problem_22() -> i64 {
  let mut f = File::open("assets/names.txt").expect("File not found!");

  let mut file_text = String::new();
  //let mut file_text = (r#""what is " going on ""#).to_string();

  f.read_to_string(&mut file_text).expect("Something went wrong reading file!");

  file_text = file_text.replace(r#"""#,"");

  let mut strings = file_text.split(",").collect::<Vec<_>>();

  strings.sort();

  let mut sum = 0_i64;
  for (index,s) in strings.iter().enumerate() {
    let value : i64 = s.bytes().map(|b| (b - ('A' as u8) + 1) as i64 ).sum();
    sum += value * (1 + (index as i64));
  }

  sum
}

fn problem_21() -> i64 {

  let mut memo : Vec<usize> = vec![0;10_000];

  for i in 2_usize..10_000 {
    memo[i] = sum_of_proper_divisors(i);
  }

  let mut result = 0;

  for i in 2_usize..10_000 {
    let other = memo[i];
    if other >= 10_000 { continue; }

    if other > i && memo[other] == i {
      result += i + other;
    }
  }

  result as i64
}


fn problem_20() -> i64 {
  let big = (2..101)
    .map( |x| x.to_biguint().unwrap() )
    .fold(1.to_biguint().unwrap(), |acc,x| acc * x );

  biguint_digits(big).into_iter().map( |x| x as i64 ).sum()
}

fn problem_19() -> i64 {
  let mut day = 365_i64;
  let mut result = 0;
  let months : [i64;12] = [ 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31 ];

  for year in 1901..2000 {
    for &days in months.iter() {
      if day % 7 == 0 {
        result += 1;
      }
      day += days;
      if days == 28 && year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        day += 1;
      }
    }
  }

  result
}


fn problem_18() -> i64 {
  let mut triangle : Vec<Vec<u32>> = vec![
    vec![75],
    vec![95,64],
    vec![17,47,82],
    vec![18,35,87,10],
    vec![20,04,82,47,65],
    vec![19,01,23,75,03,34],
    vec![88,02,77,73,07,63,67],
    vec![99,65,04,28,06,16,70,92],
    vec![41,41,26,56,83,40,80,70,33],
    vec![41,48,72,33,47,32,37,16,94,29],
    vec![53,71,44,65,25,43,91,52,97,51,14],
    vec![70,11,33,28,77,73,17,78,39,68,17,57],
    vec![91,71,52,38,17,14,91,43,58,50,27,29,48],
    vec![63,66,04,68,89,53,67,30,73,16,69,87,40,31],
    vec![04,62,98,27,23,09,70,98,73,93,38,53,60,04,23]];

  triangle.reverse();

  let mut max_totals : Vec<Vec<u32>> = vec![];

  for (i,row) in triangle.iter().enumerate() {
    let i = i as usize;
    if i == 0 {
      max_totals.push(row.clone());
      continue;
    }
    let new_row = row.iter().enumerate().map( |(j,x)| { let j = j as usize; x + std::cmp::max(max_totals[i-1][j],max_totals[i-1][j+1]) } ).collect();
    max_totals.push(new_row);
  }
  max_totals[max_totals.len()-1][0] as i64
}

//if the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.

//If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?

fn problem_17() -> i64 {
  fn tens_len(num: usize) -> usize {
    assert!(num < 10);
    match num {
      2 => "twenty".len(),
      3 => "thirty".len(),
      4 => "forty".len(),
      5 => "fifty".len(),
      6 => "sixty".len(),
      7 => "seventy".len(),
      8 => "eighty".len(),
      9 => "ninety".len(),
      _ => panic!()
    }
  }

  fn under_hundred_len(num: usize) -> usize {
    assert!(num < 100);
    match num {
      0 => 0, // Used for stuff like 40
      1 => "one".len(),
      2 => "two".len(),
      3 => "three".len(),
      4 => "four".len(),
      5 => "five".len(),
      6 => "six".len(),
      7 => "seven".len(),
      8 => "eight".len(),
      9 => "nine".len(),
      10 => "ten".len(),
      11 => "eleven".len(),
      12 => "twelve".len(),
      13 => "thirteen".len(),
      14 => "fourteen".len(),
      15 => "fifteen".len(),
      16 => "sixteen".len(),
      17 => "seventeen".len(),
      18 => "eighteen".len(),
      19 => "nineteen".len(),
      20 ... 99 => tens_len(num / 10) + under_hundred_len(num % 10),
      _ => panic!()
    }
  }

  fn number_len(num: usize) -> usize {
    if num == 1000 {
      return "onethousand".len()
    }
    let hundreds = num / 100;
    let under_hundred = num % 100;

    let mut a = under_hundred_len(hundreds);
    if a > 0 {
      a += "hundred".len();
    }
    let b = under_hundred_len(under_hundred);
    if a > 0 && b > 0 {
      a + "and".len() + b
    } else {
      a + b
    }
  }

  (1..1001).map( |i| number_len(i) as i64 ).sum()
}

fn problem_16() -> i64 {
  let num = num::pow::pow(BigUint::new(vec![2]),1000);
  biguint_digits(num).into_iter().map( |x| x as u64 ).sum::<u64>() as i64
}

fn biguint_digits(mut num : BigUint) -> Vec<i8> {
  if num == 0.to_biguint().unwrap() {
    return vec![0]
  }

  let mut digits : Vec<i8> = Vec::new();

  while num > 0.to_biguint().unwrap() {
    digits.push((&num % 10.to_biguint().unwrap()).to_i8().unwrap());
    num = num / 10.to_biguint().unwrap();
  }

  digits
}

fn problem_15() -> i64 {
  let width = 21;

  let mut memo : Vec<u64> = vec![0;width * width];

  for x in 0..width {
    memo[x] = 1;
  }

  for y in 0..width {
    memo[y * width] = 1;
  }

  for y in 1..width {
    for x in 1..width {
      memo[y * width + x] = memo[y * width + x - 1] + memo[(y - 1) * width + x];
    }
  }

  memo[width * width - 1] as i64
}

fn problem_14() -> i64 {
  fn collatz_length(num: usize, memo: &mut [u32]) -> u32 {
    if num < 2_000_000 && memo[num] != 0 {
      memo[num]
    } else {
      let next = if num % 2 == 0 {
        num / 2
      } else {
        3 * num + 1
      };
      let next_count = collatz_length(next, memo) + 1;
      if num < 2_000_000 {
        memo[num] = next_count;
      }
      next_count
    }
  }

  let mut memo : Vec<u32> = vec![0;2_000_000];

  memo[1] = 1;

  let mut biggest : u32 = 0;
  let mut biggest_count : u32 = 0;

  for num in 2_usize..1_000_000 {
    let num_count = collatz_length(num,&mut memo);
    if num_count > biggest_count {
      biggest = num as u32;
      biggest_count = num_count;
    }
  }

  biggest as i64
}

// The trick was recognizing, if you only need the first ten digits,
// chop off all digits that are too far away to change the ten
// with 100 numbers (12 digits probably? I did 14)
fn problem_13() -> i64 {
  let nums : [u64;100] = [
    3710728753390, 4637693767749, 7432498619952, 9194221336357, 2306758820753,
    8926167069662, 2811287981284, 4427422891743, 4745144573600, 7038648610584,
    6217645714185, 6490635246274, 9257586771833, 5820356532535, 8018119938482,
    3539866437282, 8651550600629, 7169388870771, 5437007057682, 5328265410875,
    3612327252500, 4587657617241, 1742370690585, 8114266041808, 5193432545172,
    6246722164843, 1573244438690, 5503768752567, 1833638482533, 8038628759287,
    7818283375799, 1672632010043, 4840309812907, 8708698755139, 5995940689575,
    6979395067965, 4105268470829, 6537860736150, 3582903531743, 9495375976510,
    8890280257173, 2526768027607, 3627021854049, 2407448690823, 9143028819710,
    3441306557801, 2305308117281, 1148769693215, 6378329949063, 6772018697169,
    9554825530026, 7608532713228, 3777424253541, 2370191327572, 2979886027225,
    1849570145487, 3829820378303, 3482954382919, 4095795306640, 2974615218550,
    4169811622207, 6246795719440, 2318970677254, 8618808822587, 1130673970830,
    8295917476714, 9762333104481, 4284628018351, 5512160354698, 3223819573432,
    7550616496518, 6217784275219, 3292418570714, 9951867143023, 7326746080059,
    7684182252467, 9714261791034, 8778364618279, 1084880252167, 7132961247478,
    6218407357239, 6662789198148, 6066182629368, 8578694408955, 6602439640990,
    6491398268003, 1673093931987, 9480937724504, 7863916702118, 1536871371193,
    4078992311553, 4488991150144, 4150312888033, 8123488067321, 8261657077394,
    2291880205877, 7715854250201, 7210783843506, 2084960398013, 5350353422647
      ];

  let result : u64 = nums.iter().sum::<u64>() / 100000;

  result as i64
}

// TODO: Probably could use properest_divisors here
fn problem_12() -> i64 {
  let mut num = 1;
  let mut i = 2;

  loop {
    num += i;

    if num_divisors(num) > 500 {
      break;
    }

    i += 1;
  }

  num as i64
}

fn num_divisors(num:i32) -> i32 {
  let mut num_divisors = 2;
  let max = (num as f64).sqrt() as i32;
  for j in 2..max+1 {
    if num % j == 0 {
      if num == max {
        num_divisors += 1;
      } else {
        num_divisors += 2;
      }
    }
  }
  num_divisors
}

fn problem_11() -> i64 {
  let grid : [[ i32; 20]; 20] = [
    [08,02,22,97,38,15,00,40,00,75,04,05,07,78,52,12,50,77,91,08],
    [49,49,99,40,17,81,18,57,60,87,17,40,98,43,69,48,04,56,62,00],
    [81,49,31,73,55,79,14,29,93,71,40,67,53,88,30,03,49,13,36,65],
    [52,70,95,23,04,60,11,42,69,24,68,56,01,32,56,71,37,02,36,91],
    [22,31,16,71,51,67,63,89,41,92,36,54,22,40,40,28,66,33,13,80],
    [24,47,32,60,99,03,45,02,44,75,33,53,78,36,84,20,35,17,12,50],
    [32,98,81,28,64,23,67,10,26,38,40,67,59,54,70,66,18,38,64,70],
    [67,26,20,68,02,62,12,20,95,63,94,39,63,08,40,91,66,49,94,21],
    [24,55,58,05,66,73,99,26,97,17,78,78,96,83,14,88,34,89,63,72],
    [21,36,23,09,75,00,76,44,20,45,35,14,00,61,33,97,34,31,33,95],
    [78,17,53,28,22,75,31,67,15,94,03,80,04,62,16,14,09,53,56,92],
    [16,39,05,42,96,35,31,47,55,58,88,24,00,17,54,24,36,29,85,57],
    [86,56,00,48,35,71,89,07,05,44,44,37,44,60,21,58,51,54,17,58],
    [19,80,81,68,05,94,47,69,28,73,92,13,86,52,17,77,04,89,55,40],
    [04,52,08,83,97,35,99,16,07,97,57,32,16,26,26,79,33,27,98,66],
    [88,36,68,87,57,62,20,72,03,46,33,67,46,55,12,32,63,93,53,69],
    [04,42,16,73,38,25,39,11,24,94,72,18,08,46,29,32,40,62,76,36],
    [20,69,36,41,72,30,23,88,34,62,99,69,82,67,59,85,74,04,36,16],
    [20,73,35,29,78,31,90,01,74,31,49,71,48,86,81,16,23,57,05,54],
    [01,70,54,71,83,51,54,69,16,92,33,48,61,43,52,01,89,19,67,48],
    ];

  let mut result = 0;

  for y in 0..20 {
    for x in 0..20 {
      if x < 17 && y < 17 {
        result = std::cmp::max(
          result,
          grid[y][x] * grid[y+1][x+1] * grid[y+2][x+2] * grid[y+3][x+3]
          )
      }
      if x > 2 && y < 17 {
        result = std::cmp::max(
          result,
          grid[y][x] * grid[y+1][x-1] * grid[y+2][x-2] * grid[y+3][x-3]
          )
      }
      if x < 17 {
        result = std::cmp::max(
          result,
          grid[y][x] * grid[y][x+1] * grid[y][x+2] * grid[y][x+3]
          )
      }
      if y < 17 {
        result = std::cmp::max(
          result,
          grid[y][x] * grid[y+1][x] * grid[y+2][x] * grid[y+3][x]
          )
      }
    }
  }

  result as i64
}


// Probably not improvable? Just gotta do the actual work of calculating primes
fn problem_10() -> i64 {
  let result : i64 = Primes::new().take_while( |&x| x < 2_000_000 ).sum();

  result
}

// - - - - - - - - - - - - - - - - - - - -

fn problem_9() -> i64 {
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

  result as i64
}

// - - - - - - - - - - - - - - - - - - - -

fn problem_8() -> i64 {
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

  result as i64
}
// - - - - - - - - - - - - - - - - - - - -

fn problem_7() -> i64 {

  let result = Primes::new().nth(10_000).expect("Infinite, shouldn't fail to find");

  result
}

// - - - - - - - - - - - - - - - - - - - -

fn problem_6() -> i64 {
  let result = (1_u64..101_u64).fold(0,|acc,x| acc + x).pow(2) - (1_u64..101_u64).fold(0,|acc,x| acc + x.pow(2));

  result as i64
}

// - - - - - - - - - - - - - - - - - - - -

fn problem_5() -> i64 {
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

  result as i64
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

fn number_is_palindrome(num:i64) -> bool {
  fn digit_at(num:i64,index:u32) -> u32 {
    ((num / 10_i64.pow(index)) % 10) as u32
  }

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

// - - - - - - - - - - - - - - - - - - - -

fn problem_2() -> i64 {

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

  result as i64
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
  fn next(&mut self) -> Option<Self::Item> {
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

fn problem_1() -> i64 {
  let mut sum = 0;

  for i in 1..1000 {
    if i % 3 == 0 || i % 5 == 0 {
      sum += i;
    }
  }

  sum
}

const ANSWERS : [i64;101] = [ 0
, 233168 , 4613732 , 6857 , 906609 , 232792560
, 25164150 , 104743 , 23514624000 , 31875000 , 142913828922
, 70600674 , 76576500 , 5537376230 , 837799 , 137846528820
, 1366 , 21124 , 1074 , 171 , 648
, 31626 , 871198282 , 4179871 , 2783915460 , 4782
, 983 , -59231 , 669171001 , 9183 , 443839
, 73682 , 45228 , 100 , 40730 , 55
, 872187 , 748317 , 932718654 , 840 , 210
, 7652413 , 162 , 16695334890 , 5482660 , 1533776805
, 5777 , 134043 , 9110846700 , 296962999629 , 997651
, 121313 , 142857 , 4075 , 376 , 249 , 972
, 153 , 26241 , 107359 , 26033 , 28684
, 127035954683 , 49 , 1322 , 272 , 661
, 7273 , 6531031914842725 , 510510 , 8319823 , 428570
, 303963552391 , 7295372 , 402 , 161667 , 190569291
, 71 , 55374 , 73162890 , 40886 , 427337
, 260324 , 425185 , 101524 , 2772 , 1818
, 1097343 , 7587457 , 743 , 1217 , 14234
, 8581146 , 1258 , 518408346 , 14316 , 24702
, 8739992577 , 18769 , 709 , 756872327473
];

#[cfg(test)]
mod tests;

