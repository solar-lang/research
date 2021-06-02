# Parsing

Split parsing like this:

`Text -> AST -> Min AST`

where one can recreate the Text from the AST. This way the AST can be used for linting and formatting easily


- instead of having a `span: Span<'a>` field everywhere, create a `Token<'a, T>` struct
        -> makes EQ MUUUUCH nicer to implement
```rust
Token<'a, T> {
    // comments preceding the token
    comments: Vec<Comment>,
    span: Span<'a>,
    content: T
}
```

