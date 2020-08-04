use std::os::raw::c_char;
use std::ffi::CStr;
use std::fmt;

/*
typedef struct APerson  {
    const char * name;
    const char * long_name;
} APerson ;

APerson *get_person(const char * name, const char * long_name);
void free_person(APerson *person);
*/

#[repr(C)]
pub struct APerson {
    name: *const c_char,
    long_name: *const c_char,
}

impl APerson {
    fn new(name: *const c_char, long_name: * const c_char) -> APerson {
        let result = APerson {
            name,
            long_name,
        };
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
        let name = unsafe {CStr::from_ptr(self.name)};
        let long_name = unsafe {CStr::from_ptr(self.long_name)};
        write!(formatter, "APerson: name: {:?}, long_name: {:?}", &name, &long_name)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn it_works() {
        // NOTE: Rust literals are not null terminated, however, a CString is.
        let name = CString::new("Name").unwrap();
        let long_name = CString::new("This is a long name").unwrap();
        let person = get_person(name.as_ptr(), long_name.as_ptr());
        free_person(Some(person));
    }
}
