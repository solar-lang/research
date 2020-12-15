
generic T, F
function map(list Vec T, f (T) -> F) -> Vec F =
    match list
        is [] then []
        or [elem, ..rest] then f elem  ++ map rest f
