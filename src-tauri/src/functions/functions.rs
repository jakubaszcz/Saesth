use crate::functions::setup::function_setup::function_setup_init;
use crate::functions::sounds::function_sound::function_sound;

pub fn functions() {
    function_setup_init();
    function_sound();
}