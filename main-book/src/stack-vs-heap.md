# Stack and Heap

These are memory constructs. There is no specific different hardware for Stack and Heap in RAM. How they are different is how stack and heap are structured in RAM. 

Stack stores fixed sized data. Example:  i32 is a type in Rust which means signed 32 bit Integer. When you define a variable as i32, then it is stored on a stack because we know the max possible number of bits that it can use, i.e 32 bits. The size is known beforehand. 
This also means that fetching stuff from memory is fast. Allocation is fast. Deleting is fast becasue we know the size is not going to increase for that particular variable in stack.


and heap stores variable sized data. The size is not known beforehand. The size requirements may increase. So when program is running or during runtime, we must to be increase or decrease the size of that memory portion. Due to this accessing data in heap is slower.  

