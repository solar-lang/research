# this example might be interesting, because it also uses tail recursion! Can be optimize this?


repeat(s String, times Int) ->  String = when times
    is 0 then ""
    is 1 then s
    is n then repeat s (n-1)


# when times <= 0 then {expr} else {expr}
# do we want this type of when?
