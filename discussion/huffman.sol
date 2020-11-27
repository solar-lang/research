Node T
| Branch
    - l Node
    - r Node
| Leaf Node

let forest = symbols | map s => Leaf s

let p = match is Node.Branch l, r => p l + p r
              or Node.Leaf v => v


function huff T (f List Node T ) -> Node T  = match f
    is [a] => a
    or [a, b] => Node.Branch la rb
    or list => list | sortby p | match
            is [a, b, ..rest] => huff [..rest, Node.Branch l a r b ]



-- Note match <expr> is    <- maybe an statement?
-- vs --
--       match is           <- always an expression (generic function)
