fn main() {
  let step = 5; // 5!
  println!("{}! is equal to {}", step, factoriel_recursive(step));
}

fn factoriel_recursive(num: i32) -> i32 {
  if(num < 2) {
    return 1
  }
  return factoriel_recursive(num-1) * num
}
