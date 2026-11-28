package com.karagionikos.biovault

import android.net.wifi.WifiManager
import android.os.Bundle
import androidx.activity.enableEdgeToEdge

class MainActivity : TauriActivity() {
  // shortcut: held while the app runs; P2 holds it only while discovering (docs/05), likely via a plugin.
  private var multicastLock: WifiManager.MulticastLock? = null

  override fun onCreate(savedInstanceState: Bundle?) {
    enableEdgeToEdge()
    super.onCreate(savedInstanceState)
    // Runtime permission on Android 17+; the prompt shows only while it isn't granted yet.
    requestPermissions(arrayOf("android.permission.ACCESS_LOCAL_NETWORK"), 0)
    val wifi = applicationContext.getSystemService(WIFI_SERVICE) as WifiManager
    multicastLock = wifi.createMulticastLock("biovault-mdns").apply {
      setReferenceCounted(false)
      acquire()
    }
  }

  override fun onDestroy() {
    multicastLock?.release()
    super.onDestroy()
  }
}
