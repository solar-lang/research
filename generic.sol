
# Example One

generic T
type Maybe = Some T | Nothing

generic T
type Maybe
|   Some T
|   Nothing


generic I, R
function map(list: List I, f: I -> R) -> List R =
    match list
        is [] then []
        or [elem, ..rest] then [f elem, .. map rest]
