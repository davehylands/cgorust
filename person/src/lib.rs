use std::ffi::CStr;
use std::fmt;
use std::os::raw::{c_char, c_int, c_void};
use std::io::{self, Write};
use std::ptr;

/*
typedef struct APerson  {
    const char * name;
    const char * long_name;
} APerson ;

APerson *get_person(const char * name, const char * long_name);
void free_person(APerson *person);

typedef void (*CallBackFuncPtr)(void);
void do_some_work(CallBackFuncPtr cb_func);

typedef void (*CallBackFuncPtr2)(void *cb_data);
void do_some_work2(CallBackFuncPtr2 cb_func, void *cb_data);
*/

#[repr(C)]
pub struct APerson {
    name: *const c_char,
    long_name: *const c_char,
}

impl APerson {
    fn new(name: *const c_char, long_name: *const c_char) -> APerson {
        let result = APerson { name, long_name };
        println!("Created {}", result);
        result
    }
}

impl Drop for APerson {
    fn drop(&mut self) {
        println!("Dropping {}", self);
    }
}

impl fmt::Display for APerson {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        let name = unsafe { CStr::from_ptr(self.name) };
        let long_name = unsafe { CStr::from_ptr(self.long_name) };
        write!(
            formatter,
            "APerson: name: {:?}, long_name: {:?}",
            &name, &long_name
        )
    }
}

// See https://doc.rust-lang.org/std/boxed/index.html#memory-layout

#[no_mangle]
pub fn get_person(name: *const c_char, long_name: *const c_char) -> Box<APerson> {
    // Box::new allocates memory from the heap and places the Person object in it.
    Box::new(APerson::new(name, long_name))
}

#[no_mangle]
pub extern "C" fn free_person(_person: Option<Box<APerson>>) {
    // When the Box is dropped at the end of this function, the memory is released.
}

// The rust FFI nomicon talks about why you should use Option<> below:
// https://doc.rust-lang.org/nomicon/ffi.html#the-nullable-pointer-optimization
//
// Note that function pointers can be wrapped in Option<> and they retain
// the size of the function pointer. However, wrapper data pointers in Option<>
// will change the size, so these can't be used as part of the C ABI.
pub type CallbackFuncPtr = Option<extern "C" fn(num: c_int)>;
pub type CallbackFuncPtr2 = Option<extern "C" fn(cb_data: *mut c_void, num: c_int)>;

#[no_mangle]
pub extern "C" fn do_some_work(cb_func: CallbackFuncPtr) {
    println!("do_some_work called");
    if let Some(cb_func) = cb_func {
        cb_func(1);
        cb_func(2);
        cb_func(3);
    } else {
        println!("do_some_work: cb_func = None");
    }
    io::stdout().flush().unwrap();
}

#[no_mangle]
pub extern "C" fn do_some_work2(cb_func: CallbackFuncPtr2, cb_data: *mut c_void) {
    println!("do_some_work2 called, sizeof(cb_func) = {}, sizeof(cb_data) = {}",
             std::mem::size_of_val(&cb_func), std::mem::size_of_val(&cb_data));
    if let Some(cb_func) = cb_func {
        if cb_data.is_null() {
            println!("do_some_work2: cb_data = null");
            cb_func(ptr::null_mut(), 1);
            cb_func(ptr::null_mut(), 2);
            cb_func(ptr::null_mut(), 3);
        } else {
            println!("do_some_work2: cb_data = {:p}", cb_data);
            cb_func(cb_data, 1);
            cb_func(cb_data, 2);
            cb_func(cb_data, 3);
        }
    } else {
        println!("do_some_work2: cb_func = null");
    }
    io::stdout().flush().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_person() {
        // NOTE: Rust literals are not null terminated, however, a CString is.
        let name = CString::new("Name").unwrap();
        let long_name = CString::new("This is a long name").unwrap();
        let person = get_person(name.as_ptr(), long_name.as_ptr());
        free_person(Some(person));
    }

    extern "C" fn my_callback(num: c_int) {
        println!("my_callback: num = {}", num);
    }

    extern "C" fn my_callback2(cb_data: *mut c_void, num: c_int) {
        if !cb_data.is_null() {
            let cb_count: &mut u32 = unsafe { &mut *(cb_data as *mut u32) };
            *cb_count += 1;
        }
        println!("my_callback: num = {}", num);
    }

    #[test]
    fn test_callback() {
        do_some_work(Some(my_callback));

        // Test by passing in some callback data
        let mut cb_count: u32 = 0;
        do_some_work2(Some(my_callback2), &mut cb_count as *mut _ as *mut c_void);
        assert_eq!(cb_count, 3);

        // Test by passing in no callback data
        cb_count = 42;
        do_some_work2(Some(my_callback2), std::ptr::null_mut());
        assert_eq!(cb_count, 42);

        // Test by passing in no callback function
        cb_count = 43;
        do_some_work2(None, std::ptr::null_mut());
        assert_eq!(cb_count, 43);
    }
}
