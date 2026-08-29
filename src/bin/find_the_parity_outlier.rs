// Topic: find_the_parity_outlier 
// Run with:  cargo run --bin find_the_parity_outlier

fn main() {
    println!("Hello from : find_the_parity_outlier");
}


fn find_outlier(values: &[i32]) -> i32 {
  let x = values[0] % 2 + values[1] % 2 + values[2] % 2;

  for i in 0..values.len() {
      match x {
          0 | 1 => {
              if values[i] % 2 == 1 {
                  return values[i];
              }
          }
          _ => {
              if values[i] % 2 == 0 {
                  return values[i];
              }
          }
      }
  }
  0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        let t1 = [2,6,8,-10,3];
        let t2 = [206847684,1056521,7,17,1901,21104421,7,1,35521,1,7781];
        let t3 = [std::i32::MAX, 0, 1];
        assert_eq!(3, find_outlier(&t1));
        assert_eq!(206847684, find_outlier(&t2));
        assert_eq!(0, find_outlier(&t3));
    }
}