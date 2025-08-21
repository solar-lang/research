## About "Call Site Generics" aka. untyped functions

at the moment we can do this:

`fun nice(name) = name ++ " is a nice person"`

and that's that.

We only know the type of name, when call it.
Implicitly we prented it's this here:

`fun nice<t>(name: t) = concat(name, " is a nice person)`

but what's the return value?
does function
`concat(t, String) -> ?`
even exist?

When refactoring this can really bite us, types don't help us as much.
-> Though I still really like, that I can just go on coding.

### It get's worse

look at `map [1, 2] times2`.

We first evaluate `[1, 2]` to be an `Array Int `.
Now look at `times2`. During evaluation, we have no idea, what parameters it's supposed to take.
We'd have to look inside `map` to figure that out!

But then, we need to find a declaration for `map(Array Int , ?)`. ~~~ That's not so easy.~~~

-> That is easy

### Solution

Search for `map(Array Int, ?)`.

#### Possibility one: Typed enough function:

Found List with only one element: `map(Array t, fun(t): r) -> ?`

Does it fit? <- How to we evalute it, and what are the results?

#### Possibility two, not so typed function:

We can't do lookup by concrete enough types.
We never can.

Found: `map(?, ?) -> ?`

How can we resolve the function now?
extend type with `Unresolved {name: "times2", scope: ...}` field, to be resolved when ready.

-> In Fact, `?` is this unresolved type
