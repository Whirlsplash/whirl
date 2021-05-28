// Copyleft (ɔ) 2021-2021 The Whirlsplash Collective
// SPDX-License-Identifier: GPL-3.0-only

const WEEK: usize = 60 * 60 * 60 * 60;
const DAY: usize = 60 * 60 * 60;
const HOUR: usize = 60 * 60;
const MIN: usize = 60;

fn make_parts(t: usize, steps: &[usize], mut accum: Vec<usize>) -> Vec<usize> {
  match steps.split_first() {
    None => accum,
    Some((s, steps)) => {
      accum.push(t / *s);
      make_parts(t % *s, steps, accum)
    }
  }
}

/// Convert a Unix (Epoch) Timestamp to a human-readable format.
#[must_use]
pub fn seconds_to_hrtime(seconds: usize) -> String {
  let word = ["week", "day", "hour", "min", "sec"];

  make_parts(seconds, &[WEEK, DAY, HOUR, MIN, 1], Vec::new())
    .iter()
    .enumerate()
    .filter_map(|(i, s)| {
      if s > &0 {
        if s > &1 {
          Some(format!("{} {}s", s, word[i]))
        } else {
          Some(format!("{} {}", s, word[i]))
        }
      } else {
        None
      }
    })
    .collect::<Vec<String>>()
    .join(", ")
}
