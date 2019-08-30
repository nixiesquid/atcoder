use super::SortOrder;

pub fn sort<T: Ord>(x: &mut [T], order: &SortOrder) -> Result<(), String> {
  if x.len().is_power_of_two() {
    match *order {
      SortOrder::Ascending => do_sort(x, true),
      SortOrder::Descending => do_sort(x, false),
    };
    Ok(())
  } else {
    Err(format!("The length of x is not a power of two. (x.len(): {})", x.len()))
  }
}

fn do_sort<T: Ord>(x: &mut [T], up: bool) {
  if x.len() > 1 {
    let mid_point = x.len() / 2;
    do_sort(&mut x[..mid_point], true);
    do_sort(&mut x[mid_point..], false);
    sub_sort(x, up);
  }
}

fn sub_sort<T: Ord>(x: &mut [T], up: bool) {
  if x.len() > 1 {
    compare_and_swap(x, up);
    let mid_point = x.len() / 2;
    sub_sort(&mut x[..mid_point], up);
    sub_sort(&mut x[mid_point..], up);
  }
}

fn compare_and_swap<T: Ord>(x: &mut [T], up: bool) {
  let mid_point = x.len() / 2;
  for i in 0..mid_point {
    if (x[i] > x[mid_point + i]) == up {
      x.swap(i, mid_point + i);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::sort;
  use crate::SortOrder::*;

  #[test]
  fn sort_u32_ascending() {
    let mut x: Vec<u32> = vec![8, 4, 2, 6, 3, 1, 5, 7];
    assert_eq!(sort(&mut x, &Ascending), Ok(()));
    assert_eq!(x, vec![1, 2, 3, 4, 5, 6, 7, 8]);
  }

  #[test]
  fn sort_u32_descending() {
    let mut x: Vec<u32> = vec![8, 4, 2, 6, 3, 1, 5, 7];
    assert_eq!(sort(&mut x, &Descending), Ok(()));
    assert_eq!(x, vec![8, 7, 6, 5, 4, 3, 2, 1]);
  }

  #[test]
  fn sort_str_ascending() {
    let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
    assert_eq!(sort(&mut x, &Ascending), Ok(()));
    assert_eq!(x, vec!["GC", "Rust", "and", "fast", "is", "memory-efficient", "no", "with"]);
  }

  #[test]
  fn sort_str_descending() {
    let mut x = vec!["Rust", "is", "fast", "and", "memory-efficient", "with", "no", "GC"];
    assert_eq!(sort(&mut x, &Descending), Ok(()));
    assert_eq!(x, vec!["with", "no", "memory-efficient", "is", "fast", "and", "Rust", "GC"]);
  }

  #[test]
  fn sort_to_fail() {
    let mut x = vec![3, 1, 2];
    assert!(sort(&mut x, &Ascending).is_err());
  }
}