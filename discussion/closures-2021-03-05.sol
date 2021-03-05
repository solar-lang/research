-- clousures in Solar
-- Problem with this: (..args) => <expr>.. where does expression end?
-- trivial solution: (..args) => (<expr>) feels redundant
-- nicer solution: (...args) { <expr> }

[1, 2, 3, 4, ] :map (n) { n^2 }

:filter (elem)  {
    elem  > 5
}

-- Pro: Easy to recognize
-- Pro: Can't be mistaken for if, because if is `if <expr> then <expr> (else <expr>)? end`
