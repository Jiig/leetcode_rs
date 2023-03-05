pub fn pivot_index(nums: Vec<i32>) -> i32 {
  let mut sum: i32 = nums.iter().sum();
  let mut tmp: i32= 0;

  for (i, e) in nums.iter().enumerate() {
    sum -= e;
    if tmp == sum { return i as i32; }
    tmp += e;
  }
  -1
}

macro_rules! run_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, pivot_index(input));
        }
    )*
    }
}

run_tests! {
  example_1: ([1,7,3,6,5,6].to_vec(), 3),
  example_2: ([1,2,3].to_vec(), -1),
}
