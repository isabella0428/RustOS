use core::{task::Poll, task::Context, pin::Pin};

use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

use crate::{println};

pub (crate) fn add_scancode(scancode: u8) {
	if let Ok(queue) = SCANCODE_QUEUE.try_get() {
		if let Err(_) = queue.push(scancode) {
			println!("WARNING: scancode queue full; dropping keyboard input");
		} else {
			WAKER.wake();
		}
	} else {
		println!("WARNING: scancode queue uninitialized");
	}
}

pub struct ScancodeStream {
	_private: (),
}

use futures_util::task::AtomicWaker;
static WAKER: AtomicWaker = AtomicWaker::new();
impl ScancodeStream {
	pub fn new() -> Self {
		SCANCODE_QUEUE.try_init_once(|| ArrayQueue::new(100)).expect("ScancodeStream::new shuld only be called once");
		ScancodeStream {_private: ()}
	}

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
		let queue = SCANCODE_QUEUE.try_get().expect("not initialized");
		match queue.pop() {
			Ok(scancode) => Poll::Ready(Some(scancode)),
			Err(crossbeam_queue::PopError) => Poll::Pending,
		}
	}
}

use futures_util::stream::Stream;
impl Stream for ScancodeStream {
	type Item = u8;

	fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
		let queue = SCANCODE_QUEUE.try_get().expect("scancode queue not initialized");

		if let Ok(scancode) = queue.pop() {
			return Poll::Ready(Some(scancode));
		}

		WAKER.register(&cx.waker());
		match queue.pop() {
			Ok(scancode) => {
				WAKER.take();
				Poll::Ready(Some(scancode))
			}
			Err(crossbeam_queue::PopError) => Poll::Pending,
		}
	}
}

use futures_util::stream::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use crate::print;


pub async fn print_keypresses() {
	let mut scancodes = ScancodeStream::new();
	let mut keyboard = Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore);

	while let Some(scancode) = scancodes.next().await {
		if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
			if let Some(key) = keyboard.process_keyevent(key_event) {
				match key {
					DecodedKey::Unicode(character) => print!("{}", character),
					DecodedKey::RawKey(key) => print!("{:?}", key),
				}
			}
		}
	}
}
