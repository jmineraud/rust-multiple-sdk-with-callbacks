#!/usr/bin/env python3

# Inspired from http://jakegoulding.com/rust-ffi-omnibus/objects/

import sys, ctypes
import os.path
from ctypes import c_char_p, c_uint32, Structure, POINTER, CFUNCTYPE

# Define a type for our callback
CBFUNC = CFUNCTYPE(None, c_uint32)  # Callback does take an unsigned int as parameters and does not return anything

# Define a place holder for the structure
# This will only be used in conjunction with the POINTER method, which creates a new type as a pointer to an existing one.
class PingPongS(Structure):
    pass

current_dir = os.path.dirname(__file__)
lib_dir = os.path.abspath(os.path.join(current_dir, '..', 'rust-lib', 'target', 'release'))

prefix = {'win32': ''}.get(sys.platform, 'lib')  # add prefix lib to all but windows 
extension = {'darwin': '.dylib', 'win32': '.dll'}.get(sys.platform, '.so')  # extension is .so for linux, .dylib for OSX and .dll for windows
lib_name = os.path.join(lib_dir, prefix + "mylib" + extension)
lib = ctypes.cdll.LoadLibrary(lib_name)  # Load the library

lib.hello_world.restype = c_char_p                                  # Returns the hello + args string
lib.hello_world.args = (c_char_p, )                                 # Takes a str as argument

lib.ping_pong_new.restype = POINTER(PingPongS)                      # Return a pointer to self
lib.ping_pong_new.argtypes = (c_uint32, c_uint32, )                 # Takes start and trigger input
lib.ping_pong_free.argtypes = (POINTER(PingPongS), )                # Equivalent to self
lib.ping_pong_set_callback.argtypes = (POINTER(PingPongS), CBFUNC)  # Equivalent to self, Callback
lib.ping_pong_ping.argtypes = (POINTER(PingPongS), )                # Equivalent to self

def hello(to):
    return lib.hello_world(str.encode(to)).decode()

class PingPong:

    def __init__(self, start, trigger):
        self.obj = lib.ping_pong_new(start, trigger)

    def __enter__(self):
        return self

    def __exit__(self, exc_type, exc_value, traceback):
        lib.ping_pong_free(self.obj)

    def set_callback(self, callback):
        lib.ping_pong_set_callback(self.obj, CBFUNC(callback))

    def ping(self):
        lib.ping_pong_ping(self.obj)


# Then we can test the library


print(hello("from the Rust native library"))

start_value = 0
trigger_value = 3
number_of_pings = 11
with PingPong(start_value, trigger_value) as pp:
    triggered_for_values = []
    def cb_(val):
        triggered_for_values.append(val)

    pp.set_callback(cb_)
    for _ in range(number_of_pings):
        pp.ping()

    print("With start at {}, trigger at {} and {} number of pings, here are the values that produced a trigger -> {}".format(
        start_value, trigger_value, number_of_pings, triggered_for_values))
