use core_graphics::display::CGDisplay;

fn main() {
    let displays = CGDisplay::active_displays();
    match displays {
        Result::Ok(displays) => {            
            for d in &displays {                
                println!("Display id is {}", d)
            }
        },
        Result::Err(err) => panic!(err)
    }
    
#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    pub fn CGConfigureDisplayOrigin(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        x: u32,
        y: u32,
    ) -> CGError;
}