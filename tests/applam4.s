setframe 0
push Lmain
call
halt
Lmain:
push undef
push 1
push undef
alloc
store 2
var 2
push 0
push _L6
set
var 2
push Lf
setframe 2
swap
call
store 2
ret
Lf:
push undef
var 0
store 3
var 3
push 3
var 3
push 0
get
setframe 3
swap
call
store 3
ret
_L6:
var 1
push 1
binary +
ret
