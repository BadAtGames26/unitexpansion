#![feature(ptr_sub_ptr)]
use engage::gamedata::{god::GodBond, unit::Unit, Gamedata, GodData, PersonData};
use skyline::hooks::InlineCtx;
use unity::{prelude::*, system::{Dictionary, List, Stack}};

#[repr(C)]
#[unity::class("App","GodBondHolder")]
pub struct GodBondHolder {
    parent: [u8; 0x10],
    pub data: &'static GodData,
    pub reliance: &'static u64,
    pub bonds: &'static Dictionary<'static, Il2CppString, GodBond>,
    pub pool: &'static PoolList<GodBond>
}

#[repr(C)]
#[unity::class("App.Pool","List")]
pub struct PoolList<T: 'static> {
    pub list: &'static List<T>,
    pub stack: &'static Stack<T>
}

#[unity::hook("App", "GodBondHolder", "Create")]
pub fn gbh_create(this: &GodBondHolder, unit: &Unit, method_info: OptionalMethod) {
    println!("GodBondHolder.Create:: Unit: {}, God: {}", unit.person.name.unwrap().to_string(), this.data.ascii_name.unwrap().to_string());
    call_original!(this, unit, method_info);
    //println!("GodBondHolder.Create:: DicLen: {}, DicCapacity: {}", this.bonds.get_count(), "?");
    println!("GodBondHolder.Create:: PoolLen: {}, PoolCapacity: {}", this.pool.list.len(), this.pool.list.capacity());
}

#[unity::hook("App", "GodBond", "Clear")]
pub fn gb_clear(this: &GodBond, method_info: OptionalMethod) {
    println!("GodBond.Clear: Start");
    call_original!(this, method_info);
    let person = PersonData::get(this.pid);
    if let Some(person) = person {
        println!("GodBond.Clear: {} {}", person.name.unwrap().to_string(), this.god.ascii_name.unwrap().to_string());
    }
    println!("GodBond.Clear: End");
}

#[skyline::hook(offset=0x02b50e98, inline)]
pub fn dictionary_hook(ctx: &mut InlineCtx) {
    ctx.registers[1].set_x(96);    
}

#[skyline::hook(offset=0x02b50ec0, inline)]
pub fn pool_hook(ctx: &mut InlineCtx) {
    ctx.registers[1].set_x(96);    
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
    skyline::install_hooks!(dictionary_hook, pool_hook, gbh_create, gb_clear); 
}
