-- dot operator

a.b

-- syntactic sugar for
(a:b)

-- which is useful for this

type XY
- f (Int) -> Int

-- now, calling
xy:f 7

-- would expand to

f(xy, 7)

-- which is undefined. TODO: MAYBE we want it this way? could be a thing. ~LATER~

-- but
xy.f 7
-- aka.
(xy:f) 7

-- is f(xy)(7)
-- WHICH IS NICE AND IS EVERYTHING I WANT FROM LIVE
