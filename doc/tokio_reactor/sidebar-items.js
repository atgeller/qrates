initSidebarItems({"fn":[["set_default","Sets `handle` as the default reactor, returning a guard that unsets it when dropped."],["with_default","Set the default reactor for the duration of the closure"]],"struct":[["Background","Handle to the reactor running on a background thread."],["DefaultGuard","Ensure that the default reactor is removed from the thread-local context when leaving the scope. This handles cases that involve panicking."],["Handle","A reference to a reactor."],["PollEvented","Associates an I/O resource that implements the `std::io::Read` and/or `std::io::Write` traits with the reactor that drives it."],["Reactor","The core reactor, or event loop."],["Registration","Associates an I/O resource with the reactor instance that drives it."],["SetFallbackError","Error returned from `Handle::set_fallback`."],["Shutdown","Future that resolves when the reactor thread has shutdown."],["Turn","Return value from the `turn` method on `Reactor`."]]});