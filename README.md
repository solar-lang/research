# solar-research
some research about language creation


## Solar
> functions give meaning to types.

--
The solar type system should feel natural and intuitive.
In order to archieve this, the leading design philosophy is that _functions give meaning to types_.
Types on the other hand are not just seen as data, but rather a description of types that are applicapable to a given state.

Coneceptually the programmer can't even access the data behind any instance of a type directly.
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