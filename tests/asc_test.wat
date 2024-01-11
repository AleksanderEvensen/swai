(module
	(type $0 (func (param i32)))
	(type $1 (func))
	(import "env" "console.log" (func $~lib/bindings/dom/console.log (param i32)))
	(memory $0 1)
	(data $0 (i32.const 1036) ",")
	(data $0.1 (i32.const 1048) "\02\00\00\00\16\00\00\00H\00e\00l\00l\00o\00 \00W\00o\00r\00l\00d")
	(export "memory" (memory $0))
	(start $~start)
	(func $~start
		i32.const 1056
		call $~lib/bindings/dom/console.log
	)
)