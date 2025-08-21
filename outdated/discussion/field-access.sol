# Field access in solar

## Field access the way it is done in C won't be a part of solar.
## instead:

--- (see also: References.sol)

type Person
- name String
## will auto derive
function name(p & Person) -> & String

type Person
+ name String
## auto derive
pub function name(p & Person) -> & String

type Person
-mut name String
## will auto derive
function name(p & Person) -> &mut String

type Person
+mut name String
## auto derive
pub function name(p & Person) -> &mut String
