function fibonacci(n: Uint) -> Uint = match n
    is 0 then 0
    or 1 then 1
    else fibonacci (n-1) + (n-2)

-- Note: no `or is`. just or. looks to much like other syntax elements otherwise