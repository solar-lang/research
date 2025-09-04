# All optional macro / Partial Macro

## Problem

- We want strict null safety on compile time.
- We want to be able to consume incomplete data.
- We might want to intermediatly work with half-filled structs.

## Solution

AllOptional macro

    @Partial
    type User
        name: String
        nickname: String
        joinedAt: Date
        -- HobbyPartial ?: Hobby
        hobbies:  List Hobby
        addr: Address


    -- generates
    type UserPartial
        name: ? String
        nickname: ? String
        joinedAt_year: ?Int
        joinedAt_day: ?Int
        addr_plz: ?String
        addr_street: ?String
        addr_country: ?String
        hobbies: ? List model.HobbyPartial
