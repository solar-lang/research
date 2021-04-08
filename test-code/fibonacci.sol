
function fib(n Int) -> Int = when n
    is 0 then 0
    is 1 then 1
    is x then fib (x-1) + fib (x-2)
