# IR

> carry the `loop` semantic into the ir.

Very nice way to deal with loop and for loops alltogether


> keep {} scopes.
This way intermediate results can be formulated like this:

```rust
let a = square(7 + 8*9)
```
<!-- note, that types are ommited -->
```asm
let a Int64
{
    tmp __1 = mul 8 9
    tmp __2 = add 7 __1
    tmp __3 = call@square __2
    store &a __3
}
```

> keep the `if`, only allow it to operate on variables/constants

```rust
if some_value > 2 == false {
    do_this()
}
```

```asm
{
    tmp __1 Bool
    {
        tmp __2 = gt some_value 2
        tmp __3 = cmp __2 false
        store &__1 __3
    }

    if __1 {
        call@do_this
    }
}
```



## Open Questions

> How do we deal with `when`
 - multiple cases for respective guards
 - don't mix array guards with other guards?

> How do we deal with `closures`?
 - need to move variables to the heap
 - need to create anonymous structs?


## Details

### Scopes

In the approuch desribe above, this simple function would result in this here:
`fib(somef(x + y*n))`

```asm
{
    let _1
    {
        let _2
        {
            let _3 = y*n
            let _4 = x + _3
            store _2 _4
        }
        let _1b = call@somef _2
        store _1 _1b
    }

    call@fib _1
}
```

Pro:
- we immediatly see, how long some value lives _at max_.
- Very primitive
- can deallocate all closed scope immeditely
Contra:
- redundant store into otherwise immutable
- declare variable, store to it later.
- need placeholder values, that hold result of variable, store to it later.
- no distinction between (immutable) value and variable
- nested

Possible solution:

```asm
let _1: i64 = {
    let _2: i64 = {
        let _3 = y * n
        x * _3
    }

    call @ somef _2
}
```

Pro:
- scoping
- less intermediate values
- assign once
Contra:
- more complex
- nested
- no deallocation for closed scopes
  -need implicit store
  - OR non linear memory usage (e.g. cant use stack)

