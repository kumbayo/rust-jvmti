extern crate libc;

use agent::Agent;
use native::{JavaVMPtr, MutString, VoidPtr, ReturnValue};

pub mod agent;
pub mod capabilities;
pub mod class;
pub mod context;
pub mod emulator;
pub mod environment;
pub mod event;
pub mod event_handler;
pub mod native;
pub mod util;

///
/// `Agent_OnLoad` is the actual entry point of the agent code and it is directly called by the
/// Java Virtual Machine.
///
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern fn Agent_OnLoad(vm: JavaVMPtr, options: MutString, reserved: VoidPtr) -> ReturnValue {

    let agent = Agent::new(vm);
//    agent.on_method_entry(Some(on_method_entry));
//    agent.on_method_exit(Some(on_method_exit));

//    agent.ready();

    return 0;
}

///
/// `Agent_OnUnload` is the exit point of the agent code. It is called when the JVM has finished
/// running and the virtual machine is unloading the agent from memory before shutting down.
/// Note: this method is also called when the JVM crashes due to an internal error.
///
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern fn Agent_OnUnload(vm: JavaVMPtr) {
}
