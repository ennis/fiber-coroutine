use std::string::{String, ToString};
use std::rc::{Rc};
use std::thread;
use std::mem::{transmute};

mod fiberapi;
use fiberapi::{SwitchToFiber, LPVOID, GetFiberData, IsThreadAFiber, ConvertThreadToFiber, GetCurrentFiber, CreateFiber};

const NULL: *mut std::os::raw::c_void = 0 as *mut std::os::raw::c_void;

//#[link(name = "fiber", kind = "static")] ???
enum YieldOperation
{
	WaitSeconds(f64),
	WaitUntilNextFrame
}

enum CoroutineStatus {
    Suspended,
    Terminated
}

struct Handle<'a>
{
	start: Box<Fn() -> () + 'a>,
	fiber: LPVOID,
	return_fiber: LPVOID,
	yield_op: YieldOperation
}

struct Coroutine<'a>
{
	handle: *mut Handle<'a>
}

fn yield_coroutine(yield_op: YieldOperation)
{
	unsafe
	{
		let c = &mut *(GetFiberData() as *mut Handle);
		//println!("Yielding");
		c.yield_op = yield_op;
		SwitchToFiber(c.return_fiber);
	}
}

extern fn start_coroutine()
{
	unsafe
	{
		let c = &*(GetFiberData() as *mut Handle);
		//println!("Into fiber: {:?}", c.fiber);
		(c.start)();
		loop {
			SwitchToFiber(c.return_fiber);
		}
	}
}

impl<'a> Coroutine<'a>
{
	fn new<F : Fn() -> () + 'a>(start: F) -> Coroutine<'a>
	{
		unsafe
		{
			let curfib: LPVOID;

			if IsThreadAFiber() == 0
			{
				curfib = ConvertThreadToFiber(NULL);
			}
			else
			{
				curfib = GetCurrentFiber();
			}

			let handle = Box::new(Handle {
				start: Box::new(start),
				fiber: NULL,
				return_fiber: curfib,
				yield_op: YieldOperation::WaitUntilNextFrame
			});

			let raw_handle : *mut Handle = std::mem::transmute(handle);
			(*raw_handle).fiber = CreateFiber(0, std::mem::transmute(start_coroutine), raw_handle as LPVOID);
			Coroutine { handle: raw_handle }
		}
	}

	fn resume(&self)
	{
		unsafe
		{
			//println!("Switching to fiber ");
			SwitchToFiber((*self.handle).fiber)
		}
		//(self.start)()
	}
}

#[test]
fn it_works() 
{
	fn return_coroutine<'a>() -> Coroutine<'a>
	{
		let y = 52;
		Coroutine::new(move || {println!("return_coroutine {}", y);})
	}

	let x = 3;

	let coro = Coroutine::new(|| {
		println!("Hello from a coroutine: {}", x);
		yield_coroutine(YieldOperation::WaitUntilNextFrame);
		println!("Hello again from a coroutine: {}", x);
	});

	let coro2 = Coroutine::new(|| {
		println!("Hello from another coroutine: {}", x);
		yield_coroutine(YieldOperation::WaitSeconds(0.001));
		println!("Hello again from a coroutine: {}", x);
	});

	let coros = [coro, coro2, return_coroutine()];

	for c in coros.iter() {
		c.resume();
	}

	println!("Main fiber");

	for c in coros.iter() {
		c.resume();
	}
}
