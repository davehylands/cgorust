use std::ffi::CStr;
use std::fmt;
use std::os::raw::{c_char, c_int /*, c_void*/};

/*
typedef struct APerson  {
    const char * name;
    const char * long_name;
} APerson ;

APerson *get_person(const char * name, const char * long_name);
void free_person(APerson *person);

typedef void (*CallBackFuncPtr)(void *cb_data);
void do_some_work(CallBackFuncPtr cb_func, void *cb_data);
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
//pub type CallbackFuncPtr = Option<extern "C" fn(cb_data: Option<*mut c_void>, num: c_int)>;
pub type CallbackFuncPtr = Option<extern "C" fn(num: c_int)>;

#[no_mangle]
pub extern "C" fn do_some_work(cb_func: CallbackFuncPtr) {
    if let Some(cb_func) = cb_func {
        cb_func(1);
        cb_func(2);
        cb_func(3);
    }
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

    /*
        extern "C" fn my_callback(cb_data: Option<*mut c_void>, num: c_int) {
            if let Some(cb_data) = cb_data {
                let cb_count: &mut u32 = unsafe { &mut *(cb_data as *mut u32) };
                *cb_count += 1;
            }
            println!("my_callback: num = {}", num);
        }
    */
    extern "C" fn my_callback(num: c_int) {
        println!("my_callback: num = {}", num);
    }

    #[test]
    fn test_callback() {
        do_some_work(Some(my_callback));
        /*
                // Test by passing in some callback data
                let mut cb_count: u32 = 0;
                do_some_work(Some(my_callback), Some(&mut cb_count as *mut _ as *mut c_void));
                assert_eq!(cb_count, 3);

                // Test by passing in no callback data
                cb_count = 42;
                do_some_work(Some(my_callback), None);
                assert_eq!(cb_count, 42);

                // Test by passing in no callback function
                cb_count = 43;
                do_some_work(None, None);
                assert_eq!(cb_count, 43);
        */
    }
}
