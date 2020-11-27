-- Example One

type Maybe T | Some T | Nothing

generic T
type Maybe
|   Some T
|   Nothing


generic I, R
function map(list List I, f I -> R) -> List R =
    match list
        is [] then []
        or [elem, ..rest] then [f elem, .. map rest]


-- can also be written as

function map(list List T, f T -> R) I, R -> List R =
    match list
        is [] then []
        or [elem, ..rest] then [f elem] ++ map rest


function subtract(left A, right A) A where Neg(A, A) and Add(A, A, A) -> A = left + -right
