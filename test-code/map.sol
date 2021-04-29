
generic T, F
function map(list Vec T, f T -> F) -> Vec F =
    when list
        is [] then []
        is [elem, ..rest] then f elem  ++  map rest f
