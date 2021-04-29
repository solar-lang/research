-- filters out elements from any sequence

type Filter (T, I)
- seq T
- filter (T) -> Bool

generic T, I
    where
    next(T) -> Option [T, I]
function filter(seq T, filter (T) -> Bool)
    -> Filter (T,I ) = Filter seq=seq filter=filter

-- GUIDE: I like this ident style better, because one can distinguish. where the nested blocks lie.
generic T, I
    where
    next(T) -> Option [T, I]
function next(f Filter (T, I)) -> Option [Fiter (T,I), I] =
    when f:seq:next
    is [_, None] then None
    is [seq, Some value] then (
        when (f:filter) value
        is true then Some [filter seq, f:filter,value]
        else filter seq (f:filter) : next
    )
