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
	printf("C.myCallback_cgo: called with num = %d\n", num);
	fflush(stdout);
	void MyCallback(int);
	MyCallback(num);
}

void MyCallback2_cgo(void *cb_data, int num)
{
	printf("C.myCallback2_cgo: called with num = %d\n", num);
	printf("C.myCallback2_cgo: called with cb_data = %lx num = %x\n", (unsigned long)cb_data, num);
	fflush(stdout);
	void MyCallback2(void *cb_data, int num);
	MyCallback2(cb_data, num);
}
*/
import "C"
