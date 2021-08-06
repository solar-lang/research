
eval(list) = eval list [].(Float)

eval(list, stack) = when (list, stack)
    is ([], stack) then stack
    is ([n]++rest, stack) if Ok(n) = float n then eval rest ([n]++stack)
    is (["add"]++rest, [a,b] ++ stack) then eval rest ([a+b]++stack)
    is (["mul"]++rest, [a,b] ++ stack) then eval rest ([a*b]++stack)
    else panic "invalid token"


