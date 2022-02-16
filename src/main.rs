use wasmtime_fiber::*;

const SIZE: usize = 2 << 20;

extern "C" {
    fn __sanitizer_start_switch_fiber(fake_stack_save: *mut usize, bottom: usize, size: usize);
    fn __sanitizer_finish_switch_fiber(
        fake_stack_save: usize,
        bottom_old: *mut usize,
        size_old: *mut usize,
    );
}

fn main() {
    unsafe {
        let ptr = libc::mmap(
            std::ptr::null_mut(),
            SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
            -1,
            0,
        );
        assert!(ptr != libc::MAP_FAILED);
        println!(
            "allocated a stack at {:?}-{:?}",
            ptr,
            ptr.cast::<u8>().add(SIZE)
        );

        {
            let stack = FiberStack::from_top_ptr(ptr.cast::<u8>().add(SIZE)).unwrap();
            let f = Fiber::<(), (), ()>::new(stack, |(), s| {
                let mut bottom_old = 0;
                let mut size_old = 0;
                __sanitizer_finish_switch_fiber(0, &mut bottom_old, &mut size_old);
                println!("\t\tin-fiber {:p}", s);
                __sanitizer_start_switch_fiber(std::ptr::null_mut(), bottom_old, size_old);
            })
            .unwrap();

            println!("starting fiber");
            __sanitizer_start_switch_fiber(std::ptr::null_mut(), ptr as usize, SIZE);
            assert!(f.resume(()).is_ok());
            __sanitizer_finish_switch_fiber(0, std::ptr::null_mut(), std::ptr::null_mut());
            println!("fiber exited, destroying fiber");
        }
        println!("releasing fiber stack");
        libc::munmap(ptr, SIZE);

        let ptr2 = libc::mmap(
            std::ptr::null_mut(),
            SIZE,
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_ANONYMOUS | libc::MAP_PRIVATE,
            -1,
            0,
        );
        assert!(ptr2 != libc::MAP_FAILED);
        if ptr != ptr2 {
            println!("got a different mmap address for second allocation");
            println!("try running program again?");
            return;
        }
        println!("got previous fiber stack as a fresh new mmap allocation");

        for i in 0..SIZE {
            std::ptr::write_volatile(ptr.cast::<u8>().add(i), 1);
        }
        libc::munmap(ptr, SIZE);
    }
}
