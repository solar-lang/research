-- GOING with this for now

type Pair S
- left S
- right S

type Tree S
|   value S
|   branches Pair (Tree S, Tree S)

generic S
    where
    add(S, S) -> S
function sum(t Tree S) -> Tree S =
    match t
    is value v then v
    is branches Pair (a, b) then sum a + sum b


generic S
function huffman(list List [S, Float32]) -> Tree S {
    let sorted List [S, Float32] = sort list

    return match sorted
        is []
}


    --- vs ---


type Tree S
| value: S
| branches:
    - left: Tree S
    - right: Tree S

generic S
function huffman(list: List Tree S) -> Tree S =
    match (sort list)
    is []   then panic "May not be empty"
    or [a]  then Tree.value a
    or [a, b] then Tree.banches (Pair left= Tree.value a    right= Tree.value b)
