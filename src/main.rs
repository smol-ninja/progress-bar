use std::{time::Duration, thread::sleep};

// String to clear up the terminal each time it prints
const CLEAR: &str = "\x1B[2J\x1B[1;1H";

// Unbounded state for progress with no limits
struct Unbounded {
  bar: &'static str
}
// Bounded state for progress with bounds and delimiter
struct Bounded {
  bound: usize,
  delims: (char, char),
  bar: &'static str
}

// progress state with type parameters: Iter and Bound
struct Progress<Iter, Bound> {
  iter: Iter,
  // unsigned integer where number of bits depend on your architecture
  i: usize,
  bound: Bound,
}

// trait to tie above data structures together 
trait ProgressDisplay : Sized {
    fn display<Iter>(&self, progress: &Progress<Iter, Self>);
}

// implements above trait for Unbounded type
impl ProgressDisplay for Unbounded {
  fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
    println!("{}", self.bar.repeat(progress.i))
  }
}

// implements above trait for Bounded type
impl ProgressDisplay for Bounded {
  fn display<Iter>(&self, progress: &Progress<Iter, Self>) {
    println!("{}{}{}{}",
              self.delims.0,
              self.bar.repeat(progress.i), 
              " ".repeat(self.bound - progress.i),
              self.delims.1)
  }
}

// defines constructure ::new()
impl<Iter> Progress<Iter, Unbounded> {
  pub fn new(iter: Iter) -> Self {
    let unbound = Unbounded {
      bar: "*"
    };
    Progress {iter, i: 0, bound: unbound}
  }
}

// implements with_bound() with type Progress 
impl<Iter> Progress<Iter, Unbounded>
where Iter: ExactSizeIterator {
  // returns Progress with Bounded type
  pub fn with_bound(mut self) -> Progress<Iter, Bounded> {
    let bound = Bounded {
      bound: self.iter.len(),
      delims: ('[', ']'),
      bar: "*",
    };
    Progress { iter: self.iter, i: self.i, bound }
  }
}

// implements with_bars_of() with type Progress with Unbounded type
impl<Iter> Progress<Iter, Unbounded> {
  pub fn with_bars_of(mut self, bar: &'static str) -> Self {
    self.bound.bar = bar;
    self
  }
}

// implements with_delims() and with_bars_of() with type Progress with Bounded type
impl<Iter> Progress<Iter, Bounded> {
  pub fn with_delims(mut self, delims: (char, char)) -> Self {
    self.bound.delims = delims;
    self
  }

  pub fn with_bars_of(mut self, bar: &'static str) -> Self {
    self.bound.bar = bar;
    self
  }
}

/* turns Progress structure into an Iterator, needs type Iterm and function next() 
as required by default Iterator interface. Doc: https://doc.rust-lang.org/stable/std/iter/trait.Iterator.html. 
This helps compiler to understand that that it can take Progress as an iterator */ 
impl<Iter, Bound> Iterator for Progress<Iter, Bound> 
where Iter: Iterator, Bound: ProgressDisplay {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
      println!("{}", CLEAR);
      self.bound.display(&self);
      self.i += 1;

      self.iter.next()
    }
}

// trait to extend progress as a method. It converts Progress::new(v.iter()) into v.iter().progress()
trait ProgressIteratorExt: Sized {
  fn progress(self) -> Progress<Self, Unbounded>;
}

// implement ProgressIteratorExt for all types Iter
impl<Iter> ProgressIteratorExt for Iter 
where Iter: Iterator {
    fn progress(self) -> Progress<Self, Unbounded> {
        Progress::new(self)
    }
}

// test
fn expensive_calculation(_n: &i32) {
  sleep(Duration::from_secs(1));
}

// test
fn main() {
  let brkts = ('<', '>');

  // Unbounded
  // for n in (0 ..).progress().with_bars_of("+") {
  //   expensive_calculation(&n);
  // }

  // Bounded
  let v = vec![1, 2, 3];
  for n in v.iter().progress().with_bound().with_delims(brkts).with_bars_of("=") {
    expensive_calculation(n);
  }
}