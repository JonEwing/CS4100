setframe 0
push Lmain
push branch
call
halt
Lmain:
push 65
push 200
swap
binary +
ret
branch:
push 5
binary /
ret