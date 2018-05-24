This is a simulation I made for CSE591 (Topics in Evolving Software) at Arizona State University, Spring 2018.
It uses genetic algorithms to evolve beetles for different tasks and has them compete with each other.

[Demo video here](https://youtu.be/W7WDCrPmXeg)

The backend is written in Rust. The frontend is in JavaScript, using d3. State updates are sent from the backend using
protocol buffers. Doing it that way was an experiment to see if the messaging system can keep up with 60FPS rendering.
It can't, at least on my systems without optimization. If I were to rewrite this from scratch I would probably either
compile the Rust down to WebAssembly and run everything in the browser, or rewrite the UI and make everything native Rust.
