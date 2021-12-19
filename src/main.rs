use inotify::{
  Inotify,
  WatchMask
};

fn main() {
  let mut inotify = Inotify::init().expect("Error initializing");
  inotify.add_watch("/dev/input", WatchMask::CREATE)
    .expect("Failed to add watch");

  let mut buffer = [0; 1024];
  let events = inotify.read_events_blocking(&mut buffer)
    .expect("Error reading events");

  for event in events {
    println!("{:?}", event);
  }
  
  println!("Hello, world!");
}
