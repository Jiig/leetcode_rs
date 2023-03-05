pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
  let mut ret = Vec::new();

  for (i, e) in nums.iter().enumerate() {
    if ret.len() == 0 {
      ret.push(nums[i])
    } else {
      let val = nums[i] + ret[i-1];
      ret.push(val);
    }
  }
  ret
}

macro_rules! run_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, expected) = $value;
            assert_eq!(expected, running_sum(input));
        }
    )*
    }
}

run_tests! {
  example_1: ([1,2,3,4].to_vec(), [1,3,6,10].to_vec()),
  example_2: ([1,1,1,1,1].to_vec(), [1,2,3,4,5].to_vec()),
}
