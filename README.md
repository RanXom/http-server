# 🧵 Rust HTTP Server with ThreadPool

A lightweight HTTP server written in Rust that uses a custom-built thread pool to handle requests concurrently. Built for educational purposes while exploring systems-level concurrency, thread management, and graceful shutdown in Rust.

---

## ✨ Features

- ✅ Multi-threaded request handling with a custom `ThreadPool`
- ✅ Graceful shutdown of threads via a messaging system
- ✅ Minimal standard library dependencies
- ✅ Based on the [Rust Book - Chapter 20](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)

---

## 🏗️ Project Structure

```
src/
├── main.rs         # Entry point; starts the server
└── lib.rs          # ThreadPool implementation
````

---

## 🚀 How It Works

### 🔁 ThreadPool

- A `ThreadPool` is created with a fixed number of worker threads.
- Each thread waits on a channel for incoming jobs (`Message::NewJob`).
- Jobs are closures sent to the workers via the `Sender`.

### 🧵 Worker Threads

- Each `Worker` owns a thread.
- When a job arrives, a worker executes it.
- On shutdown, a `Message::Terminate` is sent to each thread.
- The `Drop` implementation for `ThreadPool` ensures all threads are joined before exit.

### 📦 Message Enum

```rust
enum Message {
    NewJob(Job),
    Terminate,
}
````

This allows communication between the thread pool and its workers.

---

## 🧪 Running the Server

### ⚙️ Requirements

* Rust & Cargo (stable)
* Unix-based system or Windows with proper terminal setup

### 🔧 Usage

```bash
cargo build
cargo run
```

Server listens on: `127.0.0.1:7878`

By default, the main loop can use:

```rust
for stream in listener.incoming().take(2)
```

to process only 2 requests for demonstration and then shut down cleanly.

---

## 🛑 Graceful Shutdown

When the server exits (or goes out of scope), it sends `Message::Terminate` to all workers. Each worker exits its loop and is joined by the main thread, ensuring **no thread is left dangling**.

You will see log output like:

```
Sending terminate message to all workers.
Shutting down all workers.
Shutting down worker 0
Worker 0 was told to terminate.
...
```

---

## 🧹 Clean Code Practices

* Custom error enum (`PoolCreationError`) for safer construction
* No panics unless absolutely necessary (`unwrap()` only where safe)
* Uses `Mutex` and `Arc` for safe shared access to the job queue
* `Option<JoinHandle>` ensures we can `.take()` before `.join()` threads

---

## 🔮 Planned Features

> Want to contribute? These are great places to start:

* [ ] Handle `Ctrl+C` (SIGINT) for graceful shutdown using `ctrlc` crate
* [ ] Add a `/shutdown` HTTP route to trigger remote shutdown
* [ ] Write unit and integration tests
* [ ] Replace `unwrap()` calls with error handling
* [ ] Improve documentation with Rustdoc
* [ ] Try `threadpool` or `rayon` crate and compare performance

---

## 🧠 Learning Goals

This project demonstrates:

* How to build a thread pool from scratch
* How to use channels and mutexes for thread-safe messaging
* How to gracefully shut down a multithreaded application in Rust
* A practical use of Rust’s ownership and concurrency model

---

## 📄 License

MIT © You — Feel free to copy, modify, and distribute.

---

## 📚 References

* [The Rust Programming Language Book — Chapter 20](https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html)
