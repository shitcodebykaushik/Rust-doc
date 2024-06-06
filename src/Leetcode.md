# String to integer 
```RUST


pub struct  Solution;
fn main () {

}

impl Solution {
    

pub fn my_atoi (s : String) -> i32 {
  
  let s = s.trim_start();

  let mut chars= s.chars().peekable();

  let sign = match chars.peek(){

    Some(i) if i == &'+' => {
        chars.next();
        1
    }
 
   Some(i) if i ==& '-' => {
    chars.next();
    -1
   }

    _=> 1 ,
  };

  let mut result : i32 =0; 
  for ch in chars {
    if !ch.is_digit(10) {

        break;
    }
  let digit =sign *(ch as u8 -b'0') as i32;
  result = match result.checked_mul(10).and_then(|v| v.checked_add(digit)) {
       Some(v)=>v,
       None if sign == 1=> return  std::i32::MAX,
       None if sign == 1 => return std::i32::MIN,

    _=>unreachable!(),

  };
  }
  result
  }


}

#[cfg(test)]
 mod test {
    use crate::Solution;

      #[test]
     fn test_example() {
         assert_eq!(Solution::my_atoi("33 with the words ".into ()),33);
     }
 }










```