    fun abc() yields Int {
        yield 1
        yield 2
        yield 4
    }

->

```go
func _abc_helper(c Channel[int]) {
    c <- 1
    c <- 2
    c <- 4
}

func abc() Channel[int] {
    // default is Capacity 1
    c := newIntChannelWithCapacity(1)
    go _abc_helper(c) // c as sender
    return c // c as receiver
}

// abc can be called explicitly with own channel
func abc1(c Channel[int]) Iterator[int] {
    go _abc_helper(c) // c as sender
    return receiver(c) // we send back the channel
    // but in a way the user can't put things into it.
    // This makes optimisations easier in the future.
}
```

This will cause the solar function call

    abc

to run immediatly and put the first yield into the queue.
to run down everything immediatly we can explicitly supply a channel

    abc (channelUnbound)
