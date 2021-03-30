fun fib(n Int) Int = when n
    is 0 then 0
    is 1 then 1
    else fib (n-1) + fib (n-2)

