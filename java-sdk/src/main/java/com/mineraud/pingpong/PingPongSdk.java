package com.mineraud.pingpong;


import java.util.ArrayList;
import java.util.Arrays;
import java.util.List;
import java.util.Locale;


public class PingPongSdk {

    private static final String TAG = PingPongSdk.class.getSimpleName();

    static {
	try {
	    System.loadLibrary("mylib");
	} catch (UnsatisfiedLinkError e) {
	    System.err.println("Native code library failed to load.\n" + e);
	    System.exit(1);
	}
    }

    private static native String helloWorld(final String to);
    private static native long pingPongCreate(final int start, final int trigger);
    private static native void pingPongDestroy(final long pingPong);
    private static native void pingPongPing(final long pingPong);
    private static native void pingPongCallback(final long pingPong, PingPongCallback callback);
    
    public static String hello(String to) {
	return helloWorld(to);
    }

    private long ref;

    public PingPongSdk(int start, int trigger) {
	ref = pingPongCreate(start, trigger);
    }

    public void ping() {
	if (ref != 0) {
	    pingPongPing(ref);
	}
    }

    public void setCallback(PingPongCallback callback) {
	if (ref != 0) {
	    pingPongCallback(ref, callback);
	}
    }

    public void destroy() {
	if (ref != 0) {
	    pingPongDestroy(ref);
	    ref = 0;
	}
    }

    private static String listToString(List<Integer> list) {
	StringBuilder sb = new StringBuilder("[");
	boolean isFirst = true;
	for (int v : list) {
	    if (isFirst) isFirst = false;
	    else sb.append(",");
	    sb.append(v);
	}
	sb.append("]");
	return sb.toString();
    }
    
    public static void main(String[] args) {
	System.out.println(hello("from the native Rust library called by the Java SDK"));
	int startValue = 0;
	int triggerValue = 3;
	int numberOfPings = 11;
	final PingPongSdk pingPongSdk = new PingPongSdk(startValue, triggerValue);
	final List<Integer> triggeredForValues = new ArrayList<Integer>();
	pingPongSdk.setCallback(new PingPongCallback() {
		@Override
		public void call(int value) {
		    triggeredForValues.add(value);
		}
	    });
	for (int i = 0; i < numberOfPings; i++) {
	    pingPongSdk.ping();
	}
	pingPongSdk.destroy();
	System.out.println(String.format(Locale.ENGLISH,
					 "With start at %d, trigger at %d and %d number of pings, here are the values that produced a trigger -> %s",
					 startValue, triggerValue, numberOfPings, listToString(triggeredForValues)));
    }

}
