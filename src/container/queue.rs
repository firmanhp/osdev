pub struct Queue<T, const N: usize>
where
  T: Clone,
{
  data: [T; N],
  head: usize,
}

#[allow(dead_code)]
impl<T, const N: usize> Queue<T, N>
where
  T: Clone,
{
  pub fn size(&self) -> usize {
    self.head
  }
  pub fn empty(&self) -> bool {
    self.size() == 0
  }

  pub fn push(&mut self, data: T) {
    if self.size() == N {
      panic!("Queue overflow");
    }
    self.data[self.head] = data;
    self.head += 1;
  }

  pub fn pop(&mut self) -> T {
    if self.empty() {
      panic!("Pop on empty queue");
    }
    self.head -= 1;
    self.data[self.head].clone()
  }
}
