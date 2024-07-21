START $00
; Start with 0 and 1
MOV #00,@A
MOV #01,@B
; This is where we want to loop to
MOV @B,@C
MOV @ACC,@B
MOV @C,@A
; Output the value in A
MOV @A,@O1
JMP $04
HLT