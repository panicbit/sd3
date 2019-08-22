use core::slice;
use core::fmt::Write;
use common::Console;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    let fb_top = unsafe {
        slice::from_raw_parts_mut::<[u8; 3]>(0x18000000 as *mut _, 240 * 400)
    };

    let ref mut console = Console::new(fb_top, 400, 240);
    console.clear([0, 0, 255]);
    console.set_fg([255, 0, 0]);
    console.set_bg([0; 3]);

    writeln!(console, "arm11:");
    writeln!(console, "{}", info);

    loop {}
}
