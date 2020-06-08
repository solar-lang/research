; Vars Section for y^x
; x
; y
; return
1
; address of caller f_n-1
load $5     ; load 1
store $4    ; store 1 as return
load x
jumpifzero $6
            ; multiply ret * y
load $4     ; load return value
mul $2      ; multiply by y
store $4    ; save return value

            ; decrement x by 1
load $2     ; load x
sub $5      ; subtract 1
store $2    ; save x

jump 10     ; we could jump to 9, but this would be a no-op
