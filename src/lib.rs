#![feature(lazy_cell, ptr_sub_ptr)]
use skyline::hooks::InlineCtx;

#[skyline::hook(offset=0x02b50e98, inline)]
pub fn dictionary_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[1].x.as_mut() = 96; }     
}

#[skyline::hook(offset=0x02b50ec0, inline)]
pub fn pool_hook(ctx: &mut InlineCtx) {
    unsafe { *ctx.registers[1].x.as_mut() = 96; }    
}

#[skyline::main(name = "unitexp")]
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
            "Unit Expansion plugin has panicked at '{}' with the following message:\n{}\0",
            location,
            msg
        );

        skyline::error::show_error(
            42069,
            "
            Unit Expansion plugin has panicked! Please open the details and send a screenshot to the developer, then close the game.\n\0",
            err_msg.as_str(),
        );
    }));
    skyline::install_hooks!(dictionary_hook, pool_hook); 
}
