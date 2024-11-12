// Compute the fibonacci number at position n
fn fib(n) {
  if (n <= 1) return n
  return fib(n - 1) + fib(n - 2)
}

System.print("Fibonacci sequence:")
for (i in 0..10) {
  System.print("fib(%(_i)) = %(fib(i))")
}
