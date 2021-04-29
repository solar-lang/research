pub func encode(frames [][]u1) -> []u1 = frames :map encode :join

pub func encode(data []u1) -> []u1 =
    [0, 1, 1, 1, 1, 1, 1, 0] ++ encode_frame data ++ [0, 1, 1, 1, 1, 1, 1, 0]

func encode_frame(data []u1) -> []u1 =
    when data
    is [] => []
    is [1, 1, 1, 1, 1, ..rest] => [1, 1, 1, 1, 1, 0] ++ encode rest
    is [x, ..rest] => [x] ++ encode rest

pub func decode(data []u1 -> [][]u1 =
    when data
    is [] => []
    is [0, 1, 1, 1, 1, 1, 1, 0, ..rest] => let (rest, frame) = decode_frame rest in [frame] ++ decode rest,
    else => panic("expected 0111110 bit pattern")

func decode_frame(data []u1) -> ([]u1, []u1) =
    when data
    is [0, 1, 1, 1, 1, 1, 1, 0, ..rest] => (rest, []),
    is [1, 1, 1, 1, 1, 0, ..remaining] => let (rest, d) = decode_frame remaining in (rest, [1, 1, 1, 1, 1] ++ d),
    is [x, ..rest] = let (rest, d) = decode_frame remaining in (rest, [x] ++ d),

