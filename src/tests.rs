use super::*;

#[test]
fn test_latest() {
  //assert_eq!(problem_29(),ANSWERS[29]);
}

fn problems() -> Vec<(usize,Box<Fn()->i64>)> {
  vec![
    (1, Box::new(problem_1)),
    (2, Box::new(problem_2)),
    (3, Box::new(problem_3)),
    (4, Box::new(problem_4)),
    (5, Box::new(problem_5)),
    (6, Box::new(problem_6)),
    (7, Box::new(problem_7)),
    (8, Box::new(problem_8)),
    (9, Box::new(problem_9)),
    (10, Box::new(problem_10)),
    (11, Box::new(problem_11)),
    (12, Box::new(problem_12)),
    (13, Box::new(problem_13)),
    (14, Box::new(problem_14)),
    (15, Box::new(problem_15)),
    (16, Box::new(problem_16)),
    (17, Box::new(problem_17)),
    (18, Box::new(problem_18)),
    (19, Box::new(problem_19)),
    (20, Box::new(problem_20)),
    (21, Box::new(problem_21)),
    (22, Box::new(problem_22)),
    (23, Box::new(problem_23)),
    (24, Box::new(problem_24)),
    (25, Box::new(problem_25)),
    (26, Box::new(problem_26)),
    (27, Box::new(problem_27)),
    (28, Box::new(problem_28)),
    (29, Box::new(problem_29)),
    (30, Box::new(problem_30)),
    (31, Box::new(problem_31)),
    (32, Box::new(problem_32)),
    (33, Box::new(problem_33)),
    (34, Box::new(problem_34)),
    (35, Box::new(problem_35)),
    (36, Box::new(problem_36)),
    (37, Box::new(problem_37)),
    (38, Box::new(problem_38)),
    (39, Box::new(problem_39)),
    (40, Box::new(problem_40)),
    (41, Box::new(problem_41)),
    (42, Box::new(problem_42)),
    (43, Box::new(problem_43)),
    (44, Box::new(problem_44)),
    (45, Box::new(problem_45)),
    (46, Box::new(problem_46)),
    (47, Box::new(problem_47)),
    (48, Box::new(problem_48)),
    (49, Box::new(problem_49)),
    ]
    //,=> problem_17()),
    //,=> problem_26()),
}

#[test]
fn run_all() {
  for (i,fun) in problems() {
    assert_eq!(fun(),ANSWERS[i], "problem_{}()",i);
  }
}

use std::time::{Instant};

#[test] #[ignore]
fn benchmark_all() {
  let mut results : Vec<(usize,f64)> = vec!();
  for (i,fun) in problems() {
    let now = Instant::now();

    assert_eq!(fun(),ANSWERS[i], "problem_{}()",i);

    let elapsed = now.elapsed();
    let t = (elapsed.as_secs() as f64) + (elapsed.subsec_nanos() as f64 / 1_000_000_000.0);
    //println!("problem_{}(): {} seconds",i,t);
    results.push((i,t));
  }

  let total : f64 = results.iter().map(|&(_,t)| t).sum();
  for &(p,t) in &results {
    println!("problem_{}(): {}, {:.2}%",p,t,t*100_f64/total);
  }

  results.sort_unstable_by(|a, b| b.1.partial_cmp(&a.1).unwrap() );
  println!("\n\nWorst 10:");
  for &(p,t) in &results[0..10] {
    println!("problem_{}(): {}, {:.2}%",p,t,t*100_f64/total);
  }
}
