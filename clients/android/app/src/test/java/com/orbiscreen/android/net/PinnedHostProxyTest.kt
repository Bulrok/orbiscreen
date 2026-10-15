// Orbiscreen - PinnedHostProxyTest.kt (GPL-3.0-or-later)

package com.orbiscreen.android.net

import org.junit.Assert.assertEquals
import org.junit.Assert.assertFalse
import org.junit.Assert.assertTrue
import org.junit.Test

class PinnedHostProxyTest {
    @Test
    fun theInputWebSocketPathIsAllowedThroughTheProxy() {
        assertTrue(
            "input WebSocket must be reachable through the pinned proxy",
            pinnedProxyAllows("/input/ws"),
        )
    }

    @Test
    fun ordinaryInputAndControlPathsStayAllowed() {
        for (path in listOf("/input", "/api/control", "/api/session", "/stream", "/au")) {
            assertTrue("$path must stay allowed", pinnedProxyAllows(path))
        }
    }

    @Test
    fun unrelatedAndConfigPathsAreRefused() {
        for (path in listOf("/", "/client/config.json", "/client/index.html", "/etc/passwd")) {
            assertFalse("$path must not be allowed", pinnedProxyAllows(path))
        }
    }

    @Test
    fun upgradeHeadCarriesOnlyTheHostCredential() {
        val head = ("GET /input/ws HTTP/1.1\r\n" +
            "Host: 127.0.0.1:4567\r\n" +
            "Authorization: Bearer local-proxy-token\r\n" +
            "Upgrade: websocket\r\n" +
            "Connection: Upgrade\r\n\r\n").toByteArray(Charsets.ISO_8859_1)
        val out = withHostCredential(head, "host-credential")
        assertTrue(out.startsWith("GET /input/ws HTTP/1.1\r\n"))
        assertFalse("the proxy-local token must not reach the host", out.contains("local-proxy-token"))
        assertEquals(1, Regex("(?im)^authorization:").findAll(out).count())
        assertTrue(out.contains("Authorization: Bearer host-credential\r\n"))
        assertTrue(out.contains("Upgrade: websocket\r\n"))
        assertTrue(out.endsWith("\r\n\r\n"))
    }
}
