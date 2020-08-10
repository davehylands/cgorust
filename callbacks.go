package main

// It turns out that C can't call a Go function through a function
// pointer directly.
//
// So you need to create a "gateway" function for each function
// in go that you may want to call through a function pointer
// and use a pointer to that function instead.

/*
#include <stdio.h>

// The gateway function
void MyCallback_cgo(int num)
{
	printf("C.myCallback_cgo: called with arg = %d\n", num);
	void MyCallback(int);
	MyCallback(num);
}
*/
import "C"
