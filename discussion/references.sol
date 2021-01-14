# References

## 2 types of references will exist:

& Person
&mut Person

## the difference is, that &mut can be used with the assignment operator

NOTE: create a `set` operation? (instead of assignment)
set(ref &mut T, T)

## Auto conversion
-- (these auto conversions are the reason references deserve a special syntax)

- T will automatically be converted to &T
- T will automatically be converted to &mut T
- &mut T will automatically be converted to T
- &T will automatically be converted to T
- &mut T will automatically be converted to &T
- &T can't be used as &mut T

## But only at the beginning of a type.
- Person will be converted to &Person
- Node &Int will *not* be converted to Node Int

-- alternatively: & and &mut are types, that support special smart-pointer methods. Like deref in rust.

type W T
- value T
[W 1, W 2, W 3] :map value -- will be of type [&Int]
[W 1, W 2, W 3] :map value :map deref -- will be of type [Int]
[W 1, W 2, W 3] :map (deref value) -- will be of type [Int]
