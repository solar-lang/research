type Graph
- nodes Set Node
- edges Set Edge

type Edge
- connection [Node, Node]
- weight Float32

type Node
- name String

function contains(g Graph, elem Node) -> Boolean = (g:nodes) : find (n) -> n == elem

function oneof(l Edge, elem Node) -> Boolean = match l.connection
    is [x, _] if x == elem then true
    or [_, x] if x == elem then true
    else false
