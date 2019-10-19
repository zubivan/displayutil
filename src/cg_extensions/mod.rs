use core_graphics::display::{
    CGConfigureOption, CGDirectDisplayID, CGDisplay, CGDisplayConfigRef, CGError,
};

pub fn change_display_location(id: CGDirectDisplayID, x: i32, y: i32) {
    let display = CGDisplay::new(id);
    let config = display.begin_configuration().unwrap();
    display.change_display_location(config, x, y).unwrap();

    display
        .complete_configuration(&config, CGConfigureOption::ConfigurePermanently)
        .unwrap();
}

trait CGDisplayExt {
    fn change_display_location(
        &self,
        config: CGDisplayConfigRef,
        x: i32,
        y: i32,
    ) -> Result<(), CGError>;
}

impl CGDisplayExt for CGDisplay {
    fn change_display_location(
        &self,
        config: CGDisplayConfigRef,
        x: i32,
        y: i32,
    ) -> Result<(), CGError> {
        let result = unsafe { CGConfigureDisplayOrigin(config, self.id, x, y) };

        if result == 0 {
            Ok(())
        } else {
            Err(result)
        }
    }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
    pub fn CGConfigureDisplayOrigin(
        config: CGDisplayConfigRef,
        display: CGDirectDisplayID,
        x: i32,
        y: i32,
    ) -> CGError;
}
