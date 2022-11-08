struct Foo { x: i64, is_up: bool }

fn main() {
  println!("Struct x value {:#} and Struct is up {} ", new_foo(12, true).x);
  println!("Struct {:#}", new_foo(12, true).is_up);
}

fn new_foo(x: i64, is_up: bool) -> Foo {
  Foo { x: x, is_up: is_up }
}