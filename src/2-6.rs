#!/usr/bin/env run-cargo-script

fn main() {
  let a = [1, 2, 3];
  let b = &a;
  println!("{:p}", b);
  println!("{:?}", *b);
}