use utils::read_int_lists;

fn tail_skip(vec: &[i32], index: usize) -> Vec<i32> {
  let mut tail = vec![];
  tail.extend_from_slice(&vec[(index as i32 - 2).max(0) as usize..index]);
  tail.extend_from_slice(&vec[index + 1..]);

  return tail;
}

fn is_safe(report: &[i32], tolerance: bool) -> bool {
  fn safe_aux(report: &[i32], index: usize, tolerance: bool) -> bool {
    if !tolerance {
      return false;
    }

    for i in index.max(1) - 1..index + 2 {
      if is_safe(&tail_skip(report, i), false) {
        return true;
      }
    }

    return false;
  }

  let increasing = report[1] > report[0];

  for i in 0..report.len() - 1 {
    let level_diff = (report[i + 1] - report[i]).abs();

    if (increasing && report[i + 1] <= report[i]) || (!increasing && report[i + 1] >= report[i]) {
      return safe_aux(report, i, tolerance);
    }

    if level_diff > 3 {
      return safe_aux(report, i, tolerance);
    }
  }

  return true;
}

fn safe_count(reports: &[Vec<i32>], tolerance: bool) -> usize {
  reports.into_iter().filter(|report| is_safe(report, tolerance)).count()
}

fn main() {
  let reports = read_int_lists();

  let safe_reports = safe_count(&reports, false);
  let dampened_safe_reports = safe_count(&reports, true);

  println!("Safe reports: {safe_reports}");
  println!("Dampened safe reports: {dampened_safe_reports}");
}
