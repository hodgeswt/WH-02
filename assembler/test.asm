START $00
DEF .start
MOV #01,@A
MOV #01,@B
MOV @ACC,@O1
JMP .start
HLT