# solar-research
some research about language creation


## Solar
> functions give meaning to types.

--
The solar type system should feel natural and intuitive.
In order to archieve this, a leading design philosophy is that _functions give meaning to types_.
Types on the other hand are not just seen as structured data, but rather a description of functions that are applicapable to a given state.

Conceptually the programmer can't even access the data behind any instance of a type directly.
Instead, describing a type will automatically generate functions to derive parts of the type for us.
e.g.

```haskell
type Person
- birthday Date
- name String
```

will make the compiler create these functions

```javascript
function birthday(p &Person) -> &Date
function name(p &Person) -> &String
```

## Current Progress (implementation wise)
- [x] AST-Parser
- [ ] AST -> Semantic AST parser
- [ ] Semantic AST -> Cranelift/LLVM IR

the last part should be decoupled as its own projects, as it makes it far easier to create jit compilers.
I'm thinking about a jit compiler library called `easy-jit`. With a jet as logo. That would be nice I think.

## Notes

### solar repl

it might be nice, to have a function in the repl, to show all variables in the local scope.
For IDE stuff etc.

---

it might be nice, to have a session decoupled from a terminal.
e.g. multiple terminals can access the same session. this way one could do this script
    watch 'solar repl --session 12345 -c "current_scope : tojson" | jq'
and run it in a separate window to display all variables in the current scope

## TODO

- Include `&` and `&mut` in the grammar under section types
- Include <Ident | FullIdent>.(<Types>) in grammar
    To specify a concrete function.
    e.g.
    return fibonacci.(BigInt)
- Still unsure about this one: match::ObjectGuard should not access fields directly, but instead calls functions. (Which we desire to be O(1) field access)
