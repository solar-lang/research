-- where clause in solar
-- and generics


-- here a new Generic Parameter gets introduced, `B`
generic A
    where
    B = mul(A, Int64)
function double(value A) -> B = value * 2


-- here we specify confitions on A
generic A
    where
    add(A, A) -> A
function double2nd(value A) -> A = value + value
