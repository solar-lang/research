
-- nah

interface Writer (
    fun write(self, writable StringBuilder)
)

fun write(i: int, writable) => write (String i) writable

let writers = [Writer 7]

writers:push Writer
                (Person name="Annie")
                write=(fun (p: Person, w: Writable) => write (p.name) w
