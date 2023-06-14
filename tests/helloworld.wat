(module
	(import "std::io" "print" (func $print (param i32 i32)))
	(import "js" "mem" (memory 1))
	
	(data (i32.const 0) "Hello World from WASM")

	(data "Just A test")

	(start $main)

	(func $main (param) (result)
		memory.init 1
		i32.const 0
		i32.const 21
		call $print
	)

)