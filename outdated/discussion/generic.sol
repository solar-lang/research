-- Example One

type Maybe T | Some T | Nothing

generic T
type Maybe
|   Some T
|   Nothing


generic I, R
function map(list List I, f I -> R) -> List R =
    when list
        is [] then []
        is [elem, rest..] then [f elem] ++ map rest


-- can also be written as

function map(list, f) = when list
    is [] then []
    is [elem, rest..] then [f elem] ++ map rest

generic A
    where
    neg A -> A
    add A A -> A
function subtract(left A, right A) -> A = left + -right

