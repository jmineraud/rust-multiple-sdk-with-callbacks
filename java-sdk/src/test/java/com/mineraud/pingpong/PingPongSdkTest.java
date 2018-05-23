package com.mineraud.pingpong;


import org.junit.Assert;
import org.junit.Test;


public class PingPongSdkTest {

    @Test
    public void helloWorldTest() throws Exception {
	Assert.assertEquals("Hello world", PingPongSdk.hello("world"));
    }

}
