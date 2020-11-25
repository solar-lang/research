
function selectionSort(list List) -> List = match list
    is [] then []
    or [a] then [a]
    or [a, ..rest] then
        let (min, index) = min rest;
        if min < a then
            rest[index] = a;
            min ++ selectionSort rest
        else a ++ selectionSort rest


-- Note: this is bullshit:
function selectionSort(list List) -> List = match list
    is [] then []
    or [a] then [a]
    or [a, ..rest] then {
        let (min, index) = min rest;
        if min < a then {
            rest[index] = a;
            min ++ selectionSort rest
        } else a ++ selectionSort rest
    }

-- _if_ we want to declare that part of something isn't attached to the previous part, we'd encapsulate the previous part in ()
-- this is the way to go in solar
