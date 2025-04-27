ls tests | each {|dir|
	let assembly_file_name = ($dir.name + "\\test.asm")
	python assembler\asm6502.py $assembly_file_name
	let hex_file_name = ($dir.name + "\\test.hex");
	objcopy -I ihex $hex_file_name -O binary ($dir.name + "\\test.bin")
}