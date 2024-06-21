slint::include_modules!();
use slint::{Timer, TimerMode};
use std::time::{Duration, SystemTime};

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let mut started = false; 
    let timer = Timer::default();
    let ui_handle = ui.as_weak();

    ui.on_request_increase_value({
        move || {
            started = !started;
            println!("new started is - {started}");
            if started {
                let uiss = ui_handle.unwrap();
                let timer_start = SystemTime::now();
                Timer::start(&timer, TimerMode::Repeated, std::time::Duration::from_millis(10), move || {
                    uiss.set_minutes((timer_start.elapsed().unwrap().as_secs() / 60).try_into().unwrap());
                    uiss.set_seconds((timer_start.elapsed().unwrap().as_secs() % 60).try_into().unwrap());
                    uiss.set_mills((timer_start.elapsed().unwrap().as_millis() % 1000 / 10).try_into().unwrap());
                });
            } else {
                
                Timer::stop(&timer);
            }
        }
    });

    ui.run()
}

fn letsGo(ui: &AppWindow) -> &AppWindow {
    loop {
        let ui_handle = ui.as_weak();
        let uiHandle = ui_handle.unwrap();
        uiHandle.set_seconds(uiHandle.get_seconds() + 1);
        if uiHandle.get_seconds() > 60 {
            return ui;
        }
    }
}

// we need best time, current time, the difference between last one and best one (plus is red, minus is green)