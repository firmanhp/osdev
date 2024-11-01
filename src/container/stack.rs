use core::mem::MaybeUninit;


pub struct Stack<T, const N: usize>
where
  T: Copy,
{
  data: [MaybeUninit<T>; N],
  head: usize,
}

#[allow(dead_code)]
impl<T, const N: usize> Stack<T, N>
where
  T: Copy,
{
  pub fn size(&self) -> usize {
    self.head
  }
  pub fn empty(&self) -> bool {
    self.size() == 0
  }

  pub fn push(&mut self, data: T) {
    if self.size() == N {
      panic!("Stack overflow");
    }
    self.data[self.head] = MaybeUninit::<T>::new(data);
    self.head += 1;
  }

  pub fn pop(&mut self) -> T {
    if self.empty() {
      panic!("Pop on empty stack");
    }
    self.head -= 1;
    unsafe {
      // TODO: drop?
      self.data[self.head].assume_init()
    }
  }

  pub const fn new() -> Self {
    Self {
      data: [MaybeUninit::<T>::uninit(); N],
      head: 0,
    }
  }
}

impl<T, const N: usize> Default for Stack<T, N>
where
  T: Copy,
{
  fn default() -> Stack<T, N> {
    Stack::<T, N>::new()
  }
}
