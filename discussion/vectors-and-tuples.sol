#### also see: `no-tuples`

# Vectors & Tuples

# A vector is quite simple:
type Vector T
- elements Pointer T
- len Usize

# Now, a tuple is much more complicated,
# it  can hold all sorts of data structures.
# We'd like to type out tuple expressions like this

['hello', 76]       # for strings see ./chars-and-strings.sol

# the type of this tuple is
[String, Int]

# so this is valid

let value [String, Int] = ['hello', 76]


# What about Vectors

# Now, we'd very much like to be able to do stuff like this:

[1, 2, 3] * 2

# which evaluates to [2, 4, 6], because vectors implement

generic A, B
where add(A, B) -> C
function mul(v Vector A, scale B) -> Vector C = v:map (elem) => elem*scale

# Now, how do we get an Vector out of a tuple?
# simple: `autoconvert`

generic T
function autoconvert(t [T, T, T]) -> Vector T = ...

# with autoconvert we can automatically call all methods of vectors on tuples. Internally, the conversion will be applied automatically.
# This feature can easily get messy, so here are some rules:
# -> DEEPCOPY the input, so it's a distinct entity from the output. Don't...
# -> use sparsely



############### A PROBLEM #############

# i almost thought i had it-
# here's another problem:
# [1, 2, 3] * [4, 5, 6]
# what is this supposed to evaluate to? What if vector only implements `mul` for other vectors, and not for tuples
# continue investigating
