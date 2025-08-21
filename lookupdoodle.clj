
add(7, 79)

- (reference ("add" :int :int))
    - (lookup (:int :module :imports :buildins)) -> :int package
    <- ":add :int :int" found in :int

---


steps to look up type of value:

[]
<-> (makearray)
- (reference ("makearray")) ; no args
    - (lookup (nil :module :imports :buildins)) -> :buildins
    <- (Array ? )  ; read ? as "unresolved"

a -> a*2
<-> (makefun [a] (* a 2)) ; ++ (:scope thisscope)
- (reference ("makefun" [?]) ; one unresolved argument
    - (lookup ...) -> :buildins
    <- (fun ? -> ?)


map([], a -> a*2)
- (reference ("map" (Array ?) (fun ? -> ?)))
    - (lookup (:Array :module :imports :buildins)) -> :Array
    <- (:map (:Array t) (:fun t -> r) -> :Array r)
    ; compile as if that symbol works, and deal with array and lambda on the go
