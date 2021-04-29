-:- about haskell syntax: I LOVE that types are cursive

-- really appreciate that this feels extensible
type Person
- name String
- born Time
- gender Gender

-- This is the struct/named arguments syntax
Person name="Nils" born=(Date 1997 07 12) gender=Gender.Male

-- NOTE for now no where clause on types. Overcomplicates things and makes decisions harder.
generic T
where
    T: Add(T, T) & Sub(T, T)            -- note, no commata, for I am not using commas in Struct Literals as well
type Node
- value T
- next Node T

type Maybe T
|   Some T
|   None


-- NOTE: Not implement, complicates syntax. Open for Discussion.
-- if no new line after '=', then the first '|' can be ommited
type Gender = Male | Female | Other

-- Note: Here we have Paranthesis again.
-- struct behave like (optionally) *Named Tuples*
let me Person = Person(name = "Nils", born= Time(day= 12, month=7,1997), gender= Gender.Male)

let me Person =
        Person
            name   = "Nils"
            born   = (Date day= 12 month=7 year=1997)
            gender = Gender.Male
            hobby  = (some_function arg1 arg2)

Carl with=pony has=7


-- I prefer this:
-- struct behave more like functions with Named Arguments
Person
    name= "Nils"
    born= (Date 12 7 1997)
    gender= Gender.Male

-- Called: "named_literal"
-- -> Named expression: <ident>=<expr>
--  Note:
--      sin x y
--      sin ( x, y )
-- Also consider this case _against_ commata:
-- With , and in array
let x = [a, b, Tree left:x, right:y]
-- vs
let x = [a, b, Tree left:x right:y]

-- BUT
-- can't use <ident>:<expr>, because expression whenes list of literals
                    (e.g. sin x y)
                    function: sin
                    args: [ x, y ]
-- instead use <ident>:<literal>
-- where literal:
-- <number> | <named_literal>| <ident> | <ident_path> | (<expr>) | [<expr>, <...>>] | true | false | <string>



Idea: identifier >> '=' never is a valid expression
reason:
    this way we can write
        Tree
            left= expression
            right= otherexpression

    as
        Function: "Tree"
        Args: [
            Named: ("left", ...expression),
            Named: ("right", ...expression)
        ]



-- NO traits. Complicated
trait Add P1, P2, Output
-   add(P1, P2) -> Output

implement Add (Point, Point, Point)
- function add(a Point, b Point) -> Point = Point x= (a.x + b.x) y= (a.y + b.y)
-- note: this works, because `-` is never a postfix operator
