generic T, I
function next(sequence T) -> Option [T, I]


generic I
function next(sequence [] I) -> Option [[]I, I] =
    when sequence:len is 0 then None
    else Some [sequence:slice 1, sequence:first]
