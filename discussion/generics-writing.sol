# I hate writing generics like this
function unwrap<T>(value: T | undefined) -> T  {...}

# because _all_ the time I have to jump back and forth with the mouse cursor.
# It might just be my workflow, but I realize which generics I need while writing out the function

function unwrap(value: T | undefined) -> T with T {}

# would be easier

# for solar this would mean

generic A, B
    where
    add(A, B) -> T
    mul(T, Int) -> C
function scale(a A, b B) -> C = a + b * 2


# could become

function scale(a A, b B) -> C
with A, B
    where
    add(A, B) -> T
    mul(T, Int) -> C
    = a+b*2
