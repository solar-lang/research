use std::math::cos,sin

let x = cos π

do x *= 2.0;

function range (from:i32, to:i32) -> Stream i32 = {
    for x in from..=to {
        <- x
    }
}

// this would losely translate to this

function range(from:i32, to:i32) -> Stream i32 {
    let channel: Channel i32 = Channel<i32>.new
    thread::spawn () => {
        for x in from..=to {
            send channel x
        }
        close channel
    }

    return channel
}


    // ignore that I don't have syntax for type annotations yet.
    // I'd like to avoid paranthesis, maybe ~ works? like this: Channel ~i32::new or Pair~i32~char
    // nah.
    //
    // Generic Argumnets are separated using ::, maybe that works.
    // Vec :: i32           Pair::i32,char
    // Channel..
    // Going with thischannel | send x java like thing for now
