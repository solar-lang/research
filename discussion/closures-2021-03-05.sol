-- clousures in Solar
-- Problem here:     (..args) => <expr>.. where does expression end?
-- trivial solution: (..args) => (<expr>) feels redundant
-- similar solution: (...args) { <expr> }

[1, 2, 3, 4, ] :map (n) { n^2 }
    :filter (elem)  {
        elem  > 5
    }

-- Pro: Easy to recognize
-- Pro: Can't be mistaken for if, because if is `if <expr> then <expr> (else <expr>)? end`


# Update 30.03.2021
-- I just created Block Expressions {...}
-- this makes it hard to parse this: (a) {b}.

-- Instead

() => <expr>

[1, 2, 3, 4, ] :map (n) => n^2
--
filter list (elem)  => elem > 5
--
filter list (elem)  => elem > 5 + 5 : not
