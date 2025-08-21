
eval(list) = eval list [].(Float)

eval(list, stack) = when (list, stack)
    is ([], stack) => stack
    is ([n]++rest, stack) if Ok(n) = float n => eval rest ([n]++stack)
    is (["add"]++rest, [a,b] ++ stack) => eval rest ([a+b]++stack)
    is (["mul"]++rest, [a,b] ++ stack) => eval rest ([a*b]++stack)
    else panic "invalid token"


