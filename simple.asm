; loop beq loop
.org $1000

LDX #$06

loop dex
		 bne loop