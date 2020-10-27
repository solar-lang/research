function fibonacci(n: uint) -> uint = match n
    is 0 then 0
    or 1 then 1
    else fibonacci (n-1) + (n-2)
