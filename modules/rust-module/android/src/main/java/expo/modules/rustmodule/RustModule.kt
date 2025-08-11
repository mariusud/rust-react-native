package expo.modules.rustmodule

import expo.modules.kotlin.modules.Module
import expo.modules.kotlin.modules.ModuleDefinition
import android.util.Log
import java.net.URL

class RustModule : Module() {
  // Each module class must implement the definition function. The definition consists of components
  // that describes the module's functionality and behavior.
  // See https://docs.expo.dev/modules/module-api for more details about available components.
  
  companion object {
    // Load the native library
    init {
        Log.d("RustModule", "Loading native Rust library...")
        System.loadLibrary("native_rust_lib")
        Log.d("RustModule", "Native Rust library loaded successfully.")    
                val libraryPath = System.getProperty("java.library.path")
        Log.d("RustModule", "Available native libraries: $libraryPath")
    }
  }

  external fun rustAdd(a: Int, b: Int): Int
  

  override fun definition() = ModuleDefinition {
    // Sets the name of the module that JavaScript code will use to refer to the module. Takes a string as an argument.
    // Can be inferred from module's class name, but it's recommended to set it explicitly for clarity.
    // The module will be accessible from `requireNativeModule('RustModule')` in JavaScript.
    Name("RustModule")

    // Sets constant properties on the module. Can take a dictionary or a closure that returns a dictionary.
    Constants(
      "PI" to Math.PI
    )

    // Defines event names that the module can send to JavaScript.
    Events("onChange")

    // Defines a JavaScript synchronous function that runs the native code on the JavaScript thread.
    Function("hello") {
      "Hello world! 👋"
    }

    AsyncFunction("rustAdd") { a: Int, b: Int ->
      Log.d("RustModule", "rustAdd called with parameters: a = $a, b = $b")
      val result = rustAdd(a, b)
      Log.d("RustModule", "rustAdd result: $result")
      result
    }

    // Defines a JavaScript function that always returns a Promise and whose native code
    // is by default dispatched on the different thread than the JavaScript runtime runs on.
    AsyncFunction("setValueAsync") { value: String ->
      // Send an event to JavaScript.
      sendEvent("onChange", mapOf(
        "value" to value
      ))
    }

    // Enables the module to be used as a native view. Definition components that are accepted as part of
    // the view definition: Prop, Events.
    View(RustModuleView::class) {
      // Defines a setter for the `url` prop.
      Prop("url") { view: RustModuleView, url: URL ->
        view.webView.loadUrl(url.toString())
      }
      // Defines an event that the view can send to JavaScript.
      Events("onLoad")
    }
  }
}
