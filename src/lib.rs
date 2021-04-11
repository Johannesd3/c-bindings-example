// The implementation of the following struct will be omitted in C code, as it is not C compatible.
// Therefore everytime you use it, it must be used behind a pointer, i.e.
//
// &mylib_some_struct
// &mut mylib_some_struct
// Box<mylib_some_struct>
// Note that passing unaligned pointers or a null pointer is immediate UB.
// If you still want to allow null pointers, wrap any of the above types into an `Option`.
// 
// There are also "raw pointers", which are unsafe to use:
// *const mylib_some_struct
// *mut mylib_some_struct

pub struct mylib_some_struct {
    data: Vec<i16>,
}

#[no_mangle]
pub extern "C" fn mylib_init() -> Box<mylib_some_struct> {
    Box::new(mylib_some_struct { data: Vec::new() })
}

// &mut mylib_some_struct means that this function does _not_ take ownership of
// the struct, so the destructor of mylib_some_struct will not be run if this
// function is finished.
#[no_mangle]
pub extern "C" fn mylib_add(ptr: Option<&mut mylib_some_struct>, i: i16) {
    if let Some(s) = ptr {
        s.data.push(i);
    }
}

#[no_mangle]
pub extern "C" fn mylib_print(ptr: Option<&mylib_some_struct>) {
    if let Some(s) = ptr {
        println!("{:?}", &s.data);
    }
}
    
#[no_mangle]
pub extern "C" fn mylib_destroy(_: Option<Box<mylib_some_struct>>) {
    // mylib_some_struct will be dropped here if not None, because Box means passing ownership.
}
