pub struct Utils;

// Define GROUP_COUNTER como una variable estática global
static mut GROUP_COUNTER: u32 = 0;

impl Utils {
    pub fn generate_group_id() -> u32 {
        unsafe {
            GROUP_COUNTER += 1;
            GROUP_COUNTER
        }
    }

}