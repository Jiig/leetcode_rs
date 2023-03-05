use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
  if s.len() != t.len() {return false;}

  let mut used: HashMap<char, char> = HashMap::new();
  let mut mapped: Vec<char> = Vec::new();
  let mut newstring: Vec<char> = Vec::with_capacity(s.len());

  let (s_bytes, t_bytes) = (s.as_bytes(), t.as_bytes());
  let mut ret = true;
  for i in 0..s.len() {
    let sc = s_bytes[i] as char;
    let tc = t_bytes[i] as char;

    if !used.contains_key(&sc) && !mapped.contains(&tc) {
      println!("Haven't used this char {}", sc);
      used.insert(sc, tc);
      mapped.push(tc);
      newstring.push(tc);
    } else if used.contains_key(&sc) && used[&sc] == tc {
      println!("Have used {}, saved {}, new {}", sc, used[&sc], tc);
      newstring.push(tc);
    } else {
      ret = false
    }
  }
  println!("Used {:?}", used);

  let s: String = newstring.into_iter().collect();
  println!("{}", s);
  ret
}

macro_rules! run_tests {
    ($($name:ident: $value:expr,)*) => {
    $(
        #[test]
        fn $name() {
            let (input, input2, expected) = $value;
            assert_eq!(expected, is_isomorphic(input, input2));
        }
    )*
    }
}

run_tests! {
  example_1: ("egg".to_owned(), "add".to_owned(), true),
  example_2: ("foo".to_owned(), "bar".to_owned(), false),
  example_3: ("paper".to_owned(), "title".to_owned(), true),
  example_4: ("badc".to_owned(), "baba".to_owned(), false),
}
