-- let's imagine a type like this
type Array T
- length Usize
- capacity Usize
- elements Pointer T

generic T
function new() -> Array T

-- called new[int]
