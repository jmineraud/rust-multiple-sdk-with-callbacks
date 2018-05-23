use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

// type Callback = Box<Fn(i32) -> i32>;
/// We defined a type for the callback
/// A function that will be called by our Rust library to trigger some operation in the SDK
type Callback = extern "C" fn(u32);

/// A basic structure.
/// The aim of this structure is that each time the value is incremented via the function ping, we check if the value is a multiple of trigger
/// In case it is, the callback is executed with the current value of that triggered the callback. 
pub struct PingPong {
    value: u32,
    trigger: u32,
    callback: Option<Callback>,
}

// A basic hello world function
// No mangle ensures that the name of the function is not modified by the compilier
// Extern keyword ensures the compiler compiles the function with C convention
#[no_mangle]
pub extern fn hello_world(to: *const c_char) -> *mut c_char {
    // first we have to convert the c string (prt to const char)
    let c_str = unsafe { CStr::from_ptr(to) };
    let recipient = match c_str.to_str() {
        Err(_) => "there",
        Ok(string) => string,
    };
    // Takes our results and recreate a C str
    CString::new("Hello ".to_owned() + recipient).unwrap().into_raw()
}

impl PingPong {

    /// Constructor for the PingPong structure
    /// * start_value initialize the value
    /// * pong_trigger sets the trigger
    /// The callback is yet set
    fn new(start_value: u32, pong_trigger: u32) -> PingPong {
        PingPong {
            value: start_value,
            trigger: pong_trigger,
            callback: None
        }
    }

    /// Set the callback from the SDK to be called when the callback is called
    fn set_callback(&mut self, callback: Callback) {
        self.callback = Some(callback);
    }

    /// Increment the value and check if the callback needs to be checked
    fn ping(&mut self) {
        self.value += 1;
        // If the modulo of value and trigger is 0, trigger the callback 
        if (self.value % self.trigger) == 0 {
            self.trigger_pong_callback();
        }
    }

    /// Trigger the callback
    fn trigger_pong_callback(&self) {
        match self.callback {
            Some(ref cb) => cb(self.value), // execute the callback function if present
            None => () // Otherwise pass
        }
    }

}

#[no_mangle]
pub extern fn ping_pong_new(start: u32, pong_trigger: u32) -> *mut PingPong {
    Box::into_raw(Box::new(PingPong::new(start, pong_trigger)))
}

#[no_mangle]
pub extern fn ping_pong_free(ptr: *mut PingPong) {
    if ptr.is_null() { return }
    unsafe { Box::from_raw(ptr); }
}


#[no_mangle]
pub extern fn ping_pong_set_callback(ptr: *mut PingPong, callback: Callback) {
    let pp = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    pp.set_callback(callback);
}

#[no_mangle]
pub extern fn ping_pong_ping(ptr: *mut PingPong) {
    let pp = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    pp.ping();
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
