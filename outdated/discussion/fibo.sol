function fibonacci(n: Uint) -> Uint = match n
    is 0 then 0
    is 1 then 1
    else fibonacci (n-1) + (n-2)
