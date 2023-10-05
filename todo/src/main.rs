use chrono::{Datelike, Local, Timelike};
use slint::{Model, PlatformError, SharedString, VecModel, Weak};
use slint::{Timer, TimerMode};
use std::time::Duration;

slint::include_modules!();

fn get_date_time() -> (String, String) {
    let date_time = Local::now();
    let year = date_time.year();
    let month = date_time.month();
    let day = date_time.day();
    let hour = date_time.hour();
    let minute = date_time.minute();
    let date_str = format!("{} - {} - {}", year, month, day);
    let time_str = format!("{} : {}", hour, minute);
    (date_str, time_str)
}

fn time_event_loop(app_handler: Weak<App>) -> () {
    fn event(app_handler: Weak<App>) -> () {
        let (date_str, time_str) = get_date_time();
        app_handler.unwrap().set_date(date_str.into());
        app_handler.unwrap().set_clock(time_str.into());
    }
    //先获取时间再使用定时器获取
    let _ = event(app_handler.clone());
    let date_timer = Timer::default();
    date_timer.start(TimerMode::Repeated, Duration::from_secs(30), move || {
        let _ = event(app_handler.clone());
    });
}

fn main() -> Result<(), PlatformError> {
    let app = App::new().unwrap();
    let app_handler = app.as_weak();
    let _ = time_event_loop(app_handler.clone());
    //clear to do list
    app.on_click_clear(move || {
        let tmp_handler = app_handler.unwrap();
        let mut todo_list = tmp_handler
            .get_todo_list()
            .iter()
            .collect::<Vec<SharedString>>();

        let _ = todo_list.clear();
        let rc_list = std::rc::Rc::new(VecModel::from(todo_list));
        let _ = tmp_handler.set_todo_list(rc_list.into());
    });
    let app_handler = app.as_weak();
    app.on_click_add(move || {
        let tmp_handler = app_handler.unwrap();
        let mut todo_list = tmp_handler
            .get_todo_list()
            .iter()
            .collect::<Vec<SharedString>>();
        let input_str = tmp_handler.get_todo_input();
        let _ = todo_list.push(input_str);
        let rc_list = std::rc::Rc::new(VecModel::from(todo_list));
        let _ = tmp_handler.set_todo_list(rc_list.into());
    });

    // run -> event_loop -> end
    app.run()
}
