(module
  (import "console" "log" (func $log (param i32 i32)))
  (import "console" "printNum" (func $printNum (param i32)))
  (import "js" "mem" (memory 1))
  (data (i32.const 0) "Hello World")
  (data (i32.const 7) "Something")

  (data "TY")
  (func (export "writeHi")
	i32.const 3
	i32.const 0
	i32.const 2
	memory.init 2
	i32.const 2
	i32.const 4
	i32.add
	call $printNum
	i32.const 0
	i32.const 11
	call $log
  	))
    ;; i32.const 0  ;; pass offset 0 to log
    ;; i32.const 11  ;; pass length 2 to log
    ;; call $log))