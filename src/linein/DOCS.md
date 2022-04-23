## LineIn
Input buffering handler. Designed to interop with LineWire and multi-threaded engines

### InputQueue
A thread-safe dequeue. All methods are non-blocking

### InputBuffer\<BufferLogic>
Wrapper for passing InputQueues into BufferLogic.

### BufferLogic
Given streams of inputs, and a struct that implements BufferLogic, tell the game thread if an event has occured (eg. jump()).

A note regarding this implementation. This allows for weird multi-threaded things (which I will need to document in a bit). But generally the game-thread should be slower than the input-thread (you don't want to drop inputs), so there should be 1+ SDL2 inputs events in the buffer, which should translate into and ```event_method()```