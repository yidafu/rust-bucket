use std::fmt::*;

fn main() {
  struct Duck;

  trait Fly {
    fn fly(&self);
  }

  impl Fly for Duck {
    fn fly(&self) {
      println!("duck can fly!!")
    }
  }

  let duck = Duck;
  duck.fly();

  struct Point { x: i32, y: i32}

  impl Debug for Point {
    fn fmt(&self, f: &mut Formatter) -> Result {
      write!(f, "Debug Point {{ x: {}, y: {} }}", self.x, self.y)
    }
  }

  let p = Point{ x: 1, y: 1 };

  println!("{:?}", p);
}