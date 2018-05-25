use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

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

#[cfg(not(feature="java"))]
pub mod cb {
    pub type Callback = extern "C" fn(u32);
    pub type Env = ();
}

#[cfg(feature="java")]
pub mod cb {
    // As defined in https://github.com/prevoty/jni-rs/blob/master/example/mylib/src/lib.rs
    extern crate jni;

    // This is the interface to the JVM that we'll
    // call the majority of our methods on.
    use self::jni::JNIEnv;

    // These objects are what you should use as arguments to your native function.
    // They carry extra lifetime information to prevent them escaping this context
    // and getting used after being GC'd.
    use self::jni::objects::{GlobalRef,JValue};
    pub type Callback = GlobalRef;
    pub type Env<'a> = JNIEnv<'a>;

    pub fn to_jvalue<'a>(val: u32) -> JValue<'a> { JValue::Int(val as i32) }
    
}

use cb::{Callback,Env};

/// A basic structure.
/// The aim of this structure is that each time the value is incremented via the function ping, we check if the value is a multiple of trigger
/// In case it is, the callback is executed with the current value of that triggered the callback. 
/// TODO check if lifetime needed, use the code in https://stackoverflow.com/questions/41738049/idiomatic-way-to-store-a-closure-for-reuse
pub struct PingPong {
    value: u32,
    trigger: u32,
    callback: Option<Callback>,
}

impl PingPong {

    /// Constructor for the PingPong structure
    /// * start_value initialize the value
    /// * pong_trigger sets the trigger
    /// The callback is yet set
    fn new(start_value: u32, pong_trigger: u32) -> Self {
        PingPong {
            value: start_value,
            trigger: pong_trigger,
            callback: None
        }
    }

    /// Set the callback from the SDK to be called when the callback is called
    fn set_callback(&mut self, callback: Callback)  {
        self.callback = Some(callback);
    }

    /// Increment the value and check if the callback needs to be checked
    fn ping(&mut self, env: Env) {
        self.value += 1;
        // If the modulo of value and trigger is 0, trigger the callback 
        if (self.value % self.trigger) == 0 {
            self.trigger_pong_callback(env);
        }
    }
}

#[cfg(not(feature="java"))]
impl PingPong {
    /// Trigger the callback
    fn trigger_pong_callback(&self, _env: Env) {
        if let Some(cb) = self.callback { cb(self.value); }
    }
}

#[cfg(feature="java")]
impl PingPong {
    /// Trigger the callback
    fn trigger_pong_callback(&self, env: Env) {
        // Do the call from the Java interface
        if let Some(ref cb) = self.callback {
            env.call_method(cb.as_obj(),
                            "call",
                            "(I)V",
                            &[cb::to_jvalue(self.value)])
                            .unwrap();
        }
    }
}

#[no_mangle]
pub extern fn ping_pong_new(start: u32, pong_trigger: u32) -> *mut PingPong {
    let pp = PingPong::new(start, pong_trigger);
    Box::into_raw(Box::new(pp))
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


#[cfg(not(feature="java"))]
#[no_mangle]
pub extern fn ping_pong_ping(ptr: *mut PingPong) {
    let pp = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    pp.ping(());
}

#[cfg(feature="java")]
#[no_mangle]
pub extern fn ping_pong_ping(ptr: *mut PingPong, env: Env) {
    let pp = unsafe {
        assert!(!ptr.is_null());
        &mut *ptr
    };
    pp.ping(env);
}


/// Exposes the JNI interface for Android below
/// As in https://mozilla.github.io/firefox-browser-architecture/experiments/2017-09-21-rust-on-android.html
#[cfg(feature="java")]
#[allow(non_snake_case)]  // For the Java convention
pub mod java {

    extern crate jni;
    use super::*;
    use self::jni::JNIEnv;
    use self::jni::objects::{JObject, JClass, JString};
    use self::jni::sys::{jstring, jint, jlong};

    
    #[no_mangle]
    pub unsafe extern fn Java_com_mineraud_pingpong_PingPongSdk_helloWorld(env: JNIEnv, _: JClass, to: JString) -> jstring {
        // Our Java companion code might pass-in "world" as a string, hence the name.
        let world = hello_world(env.get_string(to).expect("invalid string").as_ptr());
        // Retake pointer so that we can use it below and allow memory to be freed when it goes out of scope.
        let world_ptr = CString::from_raw(world);
        let output = env.new_string(world_ptr.to_str().unwrap()).expect("Couldn't create java string!");
        output.into_inner()
    }

    #[no_mangle]
    pub unsafe extern fn Java_com_mineraud_pingpong_PingPongSdk_pingPongCreate(_env: JNIEnv, _: JClass,
                                                                               start: jint, trigger: jint)
                                                                               -> jlong {
        ping_pong_new(start as u32, trigger as u32) as jlong
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_mineraud_pingpong_PingPongSdk_pingPongPing(env: JNIEnv, _: JClass,
                                                                                 ping_pong_ptr: jlong) {
        let ping_pong = ping_pong_ptr as *mut PingPong;
        ping_pong_ping(ping_pong, env)
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_mineraud_pingpong_PingPongSdk_pingPongDestroy(_: JNIEnv, _: JClass,
                                                                                    ping_pong_ptr: jlong) {
        let ping_pong = ping_pong_ptr as *mut PingPong;
        ping_pong_free(ping_pong)
    }

    #[no_mangle]
    pub unsafe extern "C" fn Java_com_mineraud_pingpong_PingPongSdk_pingPongCallback(env: JNIEnv,
                                                                                     _: JClass,
                                                                                    ping_pong_ptr: jlong,
                                                                                    callback: JObject) {
        let ping_pong = ping_pong_ptr as *mut PingPong;
        let cb = env.new_global_ref(callback).unwrap();
        ping_pong_set_callback(ping_pong, cb);
    }

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
