function ageIfTom(x Person) Option Int =
    when x
    is Person age name="Tom" then Some age
    else None

generic T
function unwrap(option Option T) T =
    when option
    is Option.Some value then value
    or Option.None then panic "failed to unwrap None"
