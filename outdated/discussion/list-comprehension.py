# conclusion: Don't use list comprhension

# we'd like to support list comprehension

[f x for x in somelist]

# will be compiled to:
{
    let buffer = Buffer.with_capacity (len somelist)

    for x in somelist {
        push buffer (f x)
    }

    buffer
}

[f x for x in somelist if condition x]

# will be compiled to:
{
    let buffer = Array.new

    for x in somelist {
        if condition x {
            push buffer (f x)
        }
    }

    buffer
}


# But all of this is useful for arrays
# Though I've came to think the default should be vectors


# alternative:
# Use iterators all together!

[f x for x in somelist if condition x]

somelist :filter condition: map f


# now the latter is actually easier to write out and one has to remember less syntax


# This is were it would get interesting
[f x y for x in 0..width, y in 0..height]

# alt 1
{
    let buffer = Buffer.with_capacity (len 0..width * len 0..height)

    for x in 0..width {
        for y in 0..height {
            push buffer (f x y)
        }
    }

    buffer
}

# alt 2

0..width: map (x => x, 0..height) :map f
