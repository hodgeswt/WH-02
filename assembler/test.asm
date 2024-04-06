START $00
; Everything is empty
MOV #02,@A
MOV #01,@B
; A = 2, B = 1, ACC = 3
MOV @B,@A
; A = 1, B = 1, ACC = 2
MOV @ACC,@B
; A = 1, B = 2, ACC = 3
MOV @B,@A
; A = 2, B = 2, ACC = 4
MOV @ACC,@B
; A = 2, B = 4, ACC = 6
MOV @ACC,@O1
; O1 = 6
HLT