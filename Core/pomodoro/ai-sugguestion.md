>>> Please create a rust function that creates a timer in seperate thread

Here is an example of a Rust function that creates a timer in a separate thread:
```
use std::thread;
use std::time::Duration;

fn start_timer() {
    let duration = Duration::from_secs(10); // 10 seconds
    let timer_handle = thread::spawn(move || {
        thread::sleep(duration);
        println!("Timer expired!");
    });
}
```
This function creates a new thread using the `thread::spawn` function, and moves the `Duration` object into the new thread. The `timer_handle` variable is then used to join with the
new thread when it's finished.

The timer expires after 10 seconds, and the `println!` macro is called to print a message to the console.

Note that this is just an example function and may not be suitable for all use cases. It's important to carefully consider the specific requirements of your application and choose the
appropriate threading mechanism for your needs
