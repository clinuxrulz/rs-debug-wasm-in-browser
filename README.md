# rs-debug-wasm-in-browser
Debug Web Assembly in the Browser

This project is experimental and far from complete.

This is an attempt to provide full debug support to the browser for web assembly (against the original source language).

## How it works?

**Debug Build:** Establishes a WebSocket connection between the native rust code and the web browser through WebSocket.
Then the native rust code can be debugged normally while it sends javascript code to be executed in the browser via the WebSocket connection.

**Release Build:** Just compiles the full rust code into Web Assembly to be executed in the browser.

The strategy here is just to Remote Procudure Call the Client (Web Browser). "Debug Build" through a RPC WebSocket. "Release Build" just a direct call rather than RPC, but still have the users code unchanged when switching build types.

The Debug Build has the debug support but will run much slower, then the production build has no debug support but will run much faster.
