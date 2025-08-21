
-- I really like the idea of private default parameters, because they make tail recursion far easier to code
-- like this:

generic T, F
function map(list []T, f (T) -> F) (result []T = []) -> []F =
    match list
    is [] then result
    or [elem, ..rest] then map rest f (result ++ f elem)


-- now, the following is a little easier to read:
generic T, F
function map(list []T, f (T) -> F) -> []F =
    match list
    is [] then []
    or [elem, ..rest] then f elem ++ map elem f

-- but potentially less performant


-- honestly, the () afterwards looks pretty distinctive
