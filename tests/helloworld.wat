(module
	(import "std::io" "print" (func $print (param i32 i32)))	
	(data (i32.const 0) "Hello World from WASM")

	(start $main)

	(func $main (param) (result)
		i32.const 0
		i32.const 21
		call $print_the_text
	)

	(func $print_the_text (param i32 i32) (result i32)
		i32.const 0
		i32.const 21
		call $print
	)

)