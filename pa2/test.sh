#!/usr/bin/env bash
make -s build
if [ "$?" != 0 ]; then
	exit -1
fi

pass=( "tests/applam2.o" "tests/applam3.o" "tests/applam4.o" "tests/applam5.o" "tests/applam6.o" "tests/applam.o" "tests/array2.o" "tests/array3.o" "tests/array.o" "tests/comment.o" "tests/deadvar.o" "tests/div.o" "tests/dotwice.o" "tests/fact.o" "tests/fib-memo.o" "tests/fib.o" "tests/funptr2.o" "tests/funptr3.o" "tests/funptr.o" "tests/let1.o" "tests/let2.o" "tests/lists.o" "tests/match2.o" "tests/match.o" "tests/minus.o" "tests/multi-arg.o" "tests/mu.o" "tests/neg.o" "tests/pair.o" "tests/pairsum.o" "tests/plus.o" "tests/seq.o" "tests/times.o" )

fail=( "tests/fact2.o" "tests/heap.o" "tests/heap2.o" "tests/heap3.o" )

echo "Cases that should not have a result:"
for f in "${fail[@]}"
do
	F=$f make -s run > ${f%.o}.result 2>/dev/null
	if [ "$?" == "0" ]; then
		echo "FAILED: $f produced a result!"
	else
		echo "PASS: $f"
	fi
done

echo
echo "Cases that should have a result:"
for f in "${pass[@]}"; do
	F=$f make -s run > ${f%.o}.result 2>/dev/null
	if [ "$?" != "0" ]; then
		rm ${f%.o}.result
	fi
done
for f in "${pass[@]}"; do
	if [ ! -f ${f%.o}.expected ]; then
		echo "${f%.o}.expected not found!"
	elif [ ! -f ${f%.o}.result ]; then
		echo "FAILED: ${f%.o}.result not found!"
	elif cmp -s ${f%.o}.expected ${f%.o}.result; then
		echo "PASS: $f"
	else
		echo "FAILED: $f output wrong!"
	fi
done
