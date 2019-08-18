use core::slice;
use super::ArrayStr;
use core::fmt::Write;

// (240 * 400) / (8 * 8) = 1500
// (maximum numbers of 8x8 chars that fit on the top screen)
static mut MESSAGE: Option<ArrayStr<1500>> = None;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    unsafe {
        MESSAGE = Some(ArrayStr::new());
    }
    let message_buf = unsafe { MESSAGE.as_mut().unwrap() };

    writeln!(message_buf, "arm11:");
    writeln!(message_buf, "{}", info);

    // let empty_message = format_args!("panic!");
    // let message = info.message().unwrap_or(&empty_message);

    // write!(message_buf, "{}", message);

    let fb_top = unsafe {
        slice::from_raw_parts_mut::<[u8; 3]>(0x18000000 as *mut _, 240 * 400)
    };

    for pixel in fb_top.iter_mut() {
        *pixel = [0xFF, 0, 0];
    }

    loop {
        let mut y = 0;
        for mut line in message_buf.as_str().lines() {
            loop {
                let split = line.char_indices().nth(50).map(|(i, _)| i).unwrap_or(line.len());
 
                super::print_str(fb_top, 0, y, &line[..split]);
                y += 8;

                line = &line[split..];

                if line.is_empty() {
                    break;
                }
            }
        }
    }
}
