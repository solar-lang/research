
function fib(n Int) -> Int = match n
    is 0 then 0
    or 1 then 1
    or x then fib (x-1) + fib (x-2)
