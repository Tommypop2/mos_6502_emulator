.org $1000

; Computes the fib number double this + 1 (so Fib(13))
LDX #$06

LDA #$00
STA $1500   ; 1st number
LDA #$01
STA $1501   ; 2nd number
loop dex
		 LDA $1500  ; Load first number into accumulator and sum with 2nd
		 ADC $1501
		 STA $1500  ; Store it with the first number
		 ADC $1501  ; Add 2nd number to first number
		 STA $1501  ; Store 2nd
		 bne loop