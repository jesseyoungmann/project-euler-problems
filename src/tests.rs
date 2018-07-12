use super::*;

#[test]
fn test_latest() {
  //assert_eq!(problem_29(),ANSWERS[29]);
}

fn run_one(i: usize) -> i64 {
  let result = match i {
    1 => problem_1(),
    2 => problem_2(),
    3 => problem_3(),
    4 => problem_4(),
    5 => problem_5(),
    6 => problem_6(),
    7 => problem_7(),
    8 => problem_8(),
    9 => problem_9(),
    10 => problem_10(),
    11 => problem_11(),
    12 => problem_12(),
    13 => problem_13(),
    14 => problem_14(),
    15 => problem_15(),
    16 => problem_16(),
    //17 => problem_17(),
    18 => problem_18(),
    19 => problem_19(),
    20 => problem_20(),
    21 => problem_21(),
    22 => problem_22(),
    23 => problem_23(),
    24 => problem_24(),
    25 => problem_25(),
    //26 => problem_26(),
    27 => problem_27(),
    28 => problem_28(),
    29 => problem_29(),
    30 => problem_30(),
    31 => problem_31(),
    32 => problem_32(),
    33 => problem_33(),
    34 => problem_34(),
    35 => problem_35(),
    _ => -1
  };
  result as i64
}

#[test]
fn run_all() {
  for i in 1..35 {
    if i == 17 || i == 26 {
      continue;
    }
    assert_eq!(run_one(i),ANSWERS[i], "problem_{}()",i);
  }
}

