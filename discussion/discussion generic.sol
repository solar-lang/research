
generic S
enum Tree[S]
|   value S
|   branches
      - left Tree S
      - right Tree S


generic S
function huffman(list: List (S, float)) -> Tree S {
    let sorted = sort list

    return match sorted
        is []


    --- vs ---


enum Tree S
| value S
| branches
    - left Tree S
    - right Tree S

generic S
function huffman(list: List Tree S) -> Tree S =
    match list
    is []   then panic "May not be empty"
    or [a]  then Tree.value a
    or [a, b] then Tree.banches left: Tree.value a right: Tree.value b
