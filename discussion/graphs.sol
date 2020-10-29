#   # Thinking about Type Shortcuts
#   Type aliases
#   - [] is Array
#   - ?  is Maybe
#   - {} is Set

type Graph
- Set Node
- Set Link

type Link
- connection: Node, Node
- weight: float32

function oneof(l: Link, elem): boolean = match l
    is elem, _ then true
    or _, elem then true
    else false

type Node
- name: String
