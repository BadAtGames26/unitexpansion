#![feature(lazy_cell, ptr_sub_ptr)]
use skyline::patching::Patch;

#[skyline::main(name = "expandlim")]
pub fn main() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().unwrap();

        let msg = match info.payload().downcast_ref::<&'static str>() {
            Some(s) => *s,
            None => {
                match info.payload().downcast_ref::<String>() {
                    Some(s) => &s[..],
                    None => "Box<Any>",
                }
            },
        };


        let err_msg = format!(
            "Custom plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        skyline::error::show_error(
            420,
            "Custom plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
    // Patch two values to 96
    Patch::in_text(0x02B50E90).bytes([0x01, 0x10, 0x80, 0x52]).unwrap();
    Patch::in_text(0x02B50EB8).bytes([0x01, 0x10, 0x80, 0x52]).unwrap();

    
}
