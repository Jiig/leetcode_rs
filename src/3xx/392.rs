pub fn is_subsequence(s: String, t: String) -> bool {

  let (s_bytes, t_bytes) = (s.as_bytes(), t.as_bytes());
  let mut j = 0;
  let mut rem = 0;
  for i in 0..s.len() {
    let sc = s_bytes[i] as char;
    while j < t.len() {
      let tc = t_bytes[j] as char;
      j += 1;
      if sc == tc {
        rem += 1;
        break;
      }
    }
  }
  rem == s.len()
}

macro_rules! run_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, input2, expected) = $value;
            assert_eq!(expected, is_subsequence(input, input2));
        }
    )*
    }
}

run_tests! {
  example_1: ("abc".to_owned(), "ahbgdc".to_owned(), true),
  example_2: ("axc".to_owned(), "ahbgdc".to_owned(), false),
}