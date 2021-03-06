#![no_std]
#![no_main]
#![feature(lang_items, compiler_builtins_lib, asm)]

extern crate capsules;
extern crate compiler_builtins;

extern crate cc2538;

#[allow(unused_imports)]
#[macro_use(debug, debug_gpio, static_init)]

extern crate kernel;

#[macro_use]
pub mod io;

// How should the kernel respond when a process faults.
const FAULT_RESPONSE: kernel::process::FaultResponse = kernel::process::FaultResponse::Panic;

// Number of concurrent processes this platform supports.
const NUM_PROCS: usize = 2;
//
static mut PROCESSES: [Option<kernel::Process<'static>>; NUM_PROCS] = [None, None];

#[link_section = ".app_memory"]
// Give half of RAM to be dedicated APP memory
static mut APP_MEMORY: [u8; 0xA000] = [0; 0xA000];

pub struct Platform {
    //gpio: &'static capsules::gpio::GPIO<'static, cc2538::gpio::GPIOPin>,
   led: &'static capsules::led::LED<'static, cc2538::gpio::GPIOPin>,
}

impl kernel::Platform for Platform {
    fn with_driver<F, R>(&self, driver_num: usize, f: F) -> R
    where
        F: FnOnce(Option<&kernel::Driver>) -> R,
    {
        match driver_num {
            
            //capsules::gpio::DRIVER_NUM => f(Some(self.gpio)),
            capsules::led::DRIVER_NUM => f(Some(self.led)),
            //capsules::button::DRIVER_NUM => f(Some(self.button)),
            _ => f(None),
        }
    }
}

#[no_mangle]
pub unsafe fn reset_handler() {
    cc2538::init();

    // Setup AON event defaults
    //aon::AON_EVENT.setup();

    // Power on peripherals (eg. GPIO)
   // prcm::Power::enable_domain(prcm::PowerDomain::Peripherals);

    // Wait for it to turn on until we continue
    //while !prcm::Power::is_enabled(prcm::PowerDomain::Peripherals) {}

    // LEDs
    let led_pins = static_init!(
        [(&'static cc2538::gpio::GPIOPin, capsules::led::ActivationMode); 4],
        [
            (
                &cc2538::gpio::PC[4],
                capsules::led::ActivationMode::ActiveHigh
            ), // Red
            (
                &cc2538::gpio::PC[5],
                capsules::led::ActivationMode::ActiveHigh
            ), // Orange
            (
                &cc2538::gpio::PC[6],
                capsules::led::ActivationMode::ActiveHigh
            ), //Yellow
            (
                &cc2538::gpio::PC[7],
                capsules::led::ActivationMode::ActiveHigh
            ) //Green      
        ]
    );
    let led = static_init!(
        capsules::led::LED<'static, cc2538::gpio::GPIOPin>,
        capsules::led::LED::new(led_pins)
    );

    let mut chip = cc2538::chip::Cc2538::new();

    let openmote = Platform{ led };

    debug!("Initialization complete. Entering main loop\r");

    extern "C" {
        /// Beginning of the ROM region containing app images.
        static _sapps: u8;
    }

    kernel::process::load_processes(
        &_sapps as *const u8,
        &mut APP_MEMORY,
        &mut PROCESSES,
        FAULT_RESPONSE,
    );

    kernel::main(
        &openmote,
        &mut chip,
        &mut PROCESSES,
        &kernel::ipc::IPC::new(),
    );
}
