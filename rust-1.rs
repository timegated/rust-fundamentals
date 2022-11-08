
fn main() {
  let array: [i32; 3] = [1000, 1223, 123];
  println!("Arrays are fixed length usually in rust");
  println!("Array {:?}", array);
  for item in array.iter() {
    println!("{:?}", item);
  }
  tuple()
}

fn tuple() {
  let years: (i32, i32, bool) = (3222, 43333, true);
  // for year in iter::years {
    // }
    println!("Tuple {:?}", years);
  }