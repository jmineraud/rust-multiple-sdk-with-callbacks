// Inspired from http://jakegoulding.com/rust-ffi-omnibus/objects/

const ffi = require('ffi');

const lib = ffi.Library('../rust-lib/target/release/libmylib', {
    hello_world: ['string', ['string']],
    ping_pong_new: ['pointer', ['int', 'int']],
    ping_pong_free: ['void', ['pointer']],
    ping_pong_set_callback: ['void', ['pointer', 'pointer']],
    ping_pong_ping: ['void', ['pointer']],
});

const PingPong = function(start, trigger) {
    this.ptr = lib.ping_pong_new(start, trigger);
};

PingPong.prototype.free = function() {
    lib.ping_pong_free(this.ptr);
};

PingPong.prototype.setCallback = function(callback) {
    lib.ping_pong_set_callback(this.ptr, callback);
};

PingPong.prototype.ping = function() {
    return lib.ping_pong_ping(this.ptr);
};

hello = function(to) {
    return lib.hello_world(to);
};

console.log(hello("from the Rust native library called by the Node.js SDK"))
startValue = 0;
triggerValue = 3;
numberOfPings = 11;

const pingPong = new PingPong(startValue, triggerValue);
try {

    var triggeredForValues = [];
    var cb = ffi.Callback('void', ['int'], function(v) { triggeredForValues.push(v); });
    pingPong.setCallback(cb);

    var step;
    for (step = 0; step < numberOfPings; step++) {
	pingPong.ping();
    }
    
    console.log("With start at %d, trigger at %d and %d number of pings, here are the values that produced a trigger -> [%s]",
		startValue, triggerValue, numberOfPings, triggeredForValues.toString());
} finally {
    pingPong.free();
}
