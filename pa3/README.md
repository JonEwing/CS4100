# PA3: Runtime - Marc Baltes and Jonathan Feige
**Quick note: To see the output of the garbage collection, uncomment the lines at 265 and 418**

## Function gc
This function is called every time that the alloc instruction is executed. In the function, there are two passes to collect the garbage. The first pass takes all of the items in the heap that are directly pointed from the stack and places them in the new heap. Since some of the heaps can point to other heaps, a second pass is needed. In this pass we use a hash map to check if an object has already been copied into the new heap. If it hasn't, we can move this item into the new heap as usual. If it has, then we replace the value in the new heap with itself. Finally, we check if the garbage collector actually collected any garbage. If it did then the program can keep going but if not then there is an error because not enough space could be freed from the collection.

## Instruction: Print
This instruction is very simple. It takes the value at the stop of the stack, converts it to a character, then outputs it to stdout.

## Instruction: Spawn
When this instruction is executed, it first finds the spot in the heap that contains the pc for the function that the threads will execute. After this, an initial thread state is created with specific variables and then executed in the thread_execute function.

## Function: thread_execute
In this function, it checks if one of the threads have reached the return instruction in the program. If one has then this function does not do anything, but if it hasn't there is some things that need to be done. This function splits of the execution of the program into chunks of size q where q is the quantum for each thread. In our case, this value is always 2. Each thread will execute instructions until it has executed q instructions. If the return instruction is encountered then this function is finished and can return to the main thread. If not, this function will be called again with a new thread that is spawned at the pc of the parent thread. This will repeat until one of the threads reaches the return function.





