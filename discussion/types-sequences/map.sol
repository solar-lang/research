-- Maps any sequence
type Map (T, I)
- sequence T
- f (T) -> I

generic T, I
    where
        next(T) -> Option[T, I]
function map(sequence T, f (T) -> I) -> Map (T, I) = Map sequence=sequence f=f

generic T, I
    where
        next(T) -> Option[T, I]
function next(m Map (T, I)) -> Option [Map (T,I), I] =
    match m:sequence:next
    is [seq, Some ] value then Some [filter seq (m:f) ,(m:f) value]
    else None
