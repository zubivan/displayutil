use core_graphics::display::{
    CGConfigureOption, CGDirectDisplayID, CGDisplay,
};

pub fn change_display_origin(id: CGDirectDisplayID, x: i32, y: i32) {
    let display = CGDisplay::new(id);
    let config = display.begin_configuration().unwrap();
    display.configure_display_origin(&config, x, y).unwrap();

    display
        .complete_configuration(&config, CGConfigureOption::ConfigurePermanently)
        .unwrap();
}
