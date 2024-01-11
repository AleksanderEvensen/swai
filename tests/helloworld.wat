(module
	(import "std::io" "print" (func $print (param i32 i32)))
	(data (i32.const 1) "Hello World from WASM")
	
	(start $main)

	(func $main (param) (result)
		(call $print_the_text (i32.const 0) (i32.const 0) )
	)
	(func $print_the_text (param $a i32) (param $b i32) (result)
		(call $print (local.get $a) (local.get $b))
	)
)