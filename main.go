//
// main.go
// Copyright (C) 2019 Tim Hughes
//
// Distributed under terms of the MIT license.
//
package main

/*
#cgo LDFLAGS: -Lperson/target/debug -lperson
#include "person.h"

// Forward declaration of the gateway function.
void MyCallback_cgo(int num);
void MyCallback2_cgo(void *cb_data, int num);
*/
import "C"
import (
	"fmt"
	"unsafe"

	gopointer "github.com/mattn/go-pointer"
)

type (
	Person C.struct_APerson
)

func (p *Person) Name() string {
	return C.GoString(p.name)
}

func (p *Person) LongName() string {
	return C.GoString(p.long_name)
}

func GetPerson(name string, long_name string) *Person {
	return (*Person)(C.get_person(C.CString(name), C.CString(long_name)))
}

func FreePerson(person *Person) {
	C.free_person((*C.struct_APerson)(person))
}

//export MyCallback
func MyCallback(num C.int) {

	fmt.Printf("my_callback: num = %d\n", num)
}

//export MyCallback2
func MyCallback2(cb_data unsafe.Pointer, num C.int) {
	cbCountPtr := gopointer.Restore(cb_data).(*C.int)
	fmt.Printf("my_callback2: num = %d cb_count = %d\n", num, *cbCountPtr)
	*cbCountPtr = *cbCountPtr + 1
	fmt.Printf("my_callback2: num = %d cb_count = %d\n", num, *cbCountPtr)
}

func DoSomeWork() {
	C.do_some_work((C.CallBackFuncPtr)(unsafe.Pointer(C.MyCallback_cgo)))
}

func DoSomeWork2() {
	var cb_count C.int = 0

	p := gopointer.Save(&cb_count)
	defer gopointer.Unref(p)

	fmt.Printf("DoSomeWork2: p = %p, sizeof(p) = %d\n", p, unsafe.Sizeof(p))

	C.do_some_work2((C.CallBackFuncPtr2)(unsafe.Pointer(C.MyCallback2_cgo)), p)

	fmt.Printf("DoSomeWork2: cb_count = %d\n", cb_count)
}

func main() {
	var f *Person
	f = GetPerson("tim", "tim hughes")
	fmt.Printf("Hello Go rust world: My name is %s, %s.\n", C.GoString(f.name), C.GoString(f.long_name))
	fmt.Printf("Hello Go ruat world: My name is %s, %s.\n", f.Name(), f.LongName())
	FreePerson(f)

	DoSomeWork()
	DoSomeWork2()
}
