
-- simple generics in structs
type List T
- value: T
- next: Option T

type Vec T
- capacity: Usize
- len: Usize
- pointer: Pointer T

-- now, how to we specify the type List<Vec<Byte>>?
function packages() -> List Vec Byte = ...

-- What if we have multiple generic types?
type Either (A, B)
| Left A
| Right B

-- What's the signature of List<Either<A, B>>?
-- (it can't be `List Either A B`, which would be
--  parsed as List<Either<A<B>>>)

function rows() -> List Either (Int, String) = ...


-- How do we specify generic functions?
-- Lets start with non-generic function signatures first

function double(value: Int64) -> Int64 = value * 2

-- Now, let's keep this simple
-- only input parameters after `generic` keyword
generic T
    where R = mul(T, Int64)
function double(value: T) -> R = value * 2

-- may be called like this:
-- double 123
-- or with the full identifier name:
-- double$Int64 123

-- Example:
-- double$Int64 will be compiled to
-- Int64 double__Int64(Int64 value);

-- the generic definition should be equivalent to
function double(value) = value * 2
-- the reason is, that this whole `where` stuff can and should be figured out by the compiler (Opinion)

-- now, more complex generics
generic In
 where
 N1 = mul(In, Int64)
 R = add(N1, Int64)
function prepend_one(value: In) -> R = value*2 + 1

-- one can see, that the generic parameters for the input are the only relevant ones
-- rather than interfaces, only function definitions are needed
-- it's a rather primitive type system
