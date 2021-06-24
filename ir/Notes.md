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
