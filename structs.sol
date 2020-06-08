
-- really appreciate that this feels extensible
struct Person
- name String
- born Time
- gender Gender

generic T
where
    T: Add(T, T) & Sub(T, T)            -- note, no commata, for I am not using commas in Struct Literals as well
struct Node
- value T
- next Node T

enum Maybe T
|   Some T
|   None

-- if no new line after '=', then the first '|' can be ommited
enum Gender = Male | Female

-- Note: Here we have Paranthesis again.
-- struct behave like (optionally) *Named Tuples*
let me: Person = Person(name = "Nils", born= Time(day= 12, month=7,1997), gender= Gender.Male)

let me: Person =
        Person
            name   : "Nils"
            born   : (Date day: 12 month:7 year:1997)
            gender : Gender.Male
            hobby  : (some_function arg1 arg2)

Carl with: pony has: 7


-- I prefer this:
-- struct behave more like functions with Named Arguments
Person
    name: "Nils"
    born: (Date 12 7 1997)
    gender: Gender.Male

-- Called: "named_literal"
-- -> Named expression: <ident>:<expr>
--  Note:
--      sin x y
--      sin ( x, y )
Also consider this case _against_ commata:
-- With , and in array
let x = [a, b, Tree left:x, right:y]
-- vs
let x = [a, b, Tree left:x right:y]

-- BUT
-- can't use <ident>:<expr>, because expression matches list of literals
                    (e.g. sin x y)
                    function: sin
                    args: [ x, y ]
-- instead use <ident>:<literal>
-- where literal:
-- <number> | <named_literal>| <ident> | <ident_path> | (<expr>) | [<expr>, <...>>] | true | false | <string>



Idea: identifier >> ':' never is a valid expression
reason:
    this way we can write
        Tree
            left: expression
            right: otherexpression

    as
        Function: "Tree"
        Args: [
            Named: ("left", ...expression),
            Named: ("right", ...expression)
        ]



trait Add P1, P2, Output
-   add(P1, P2) -> Output

implement Add (Point, Point, Point)
- function add(a: Point, b: Point) -> Point = Point x: (a.x + b.x) y: (a.y + b.y)
-- note: this works, because `-` is never a postfix operator
