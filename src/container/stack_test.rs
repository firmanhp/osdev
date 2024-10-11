use super::*;

#[test]
fn test_stack_push() {
  let mut stack = Stack::<u8, 64>::new();

  assert!(stack.empty());
  stack.push(5);
  assert_eq!(stack.pop(), 5);

  assert!(stack.empty());
  stack.push(1);
  stack.push(2);
  stack.push(3);
  assert_eq!(stack.pop(), 3);
  assert_eq!(stack.pop(), 2);
  assert_eq!(stack.pop(), 1);

  assert!(stack.empty());
}

#[test]
fn test_int_parse() {
  let mut stack = Stack::<u8, 64>::new();
  let mut int = 123456;

  // Fetch digits
  while int > 0 {
    let digit: u8 = (int % 10) as u8;
    stack.push(('0' as u8) + digit);
    int /= 10;
  }
  assert_eq!(stack.pop(), '1' as u8);
  assert_eq!(stack.pop(), '2' as u8);
  assert_eq!(stack.pop(), '3' as u8);
  assert_eq!(stack.pop(), '4' as u8);
  assert_eq!(stack.pop(), '5' as u8);
  assert_eq!(stack.pop(), '6' as u8);
}

#[test]
fn test_int_parse_2() {
  let mut stack = Stack::<u8, 64>::new();
  let mut int = 2;

  // Fetch digits
  while int > 0 {
    let digit: u8 = (int % 10) as u8;
    stack.push(('0' as u8) + digit);
    int /= 10;
  }
  assert_eq!(stack.pop(), '2' as u8);
}
