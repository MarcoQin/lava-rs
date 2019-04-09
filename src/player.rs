extern crate libc;
use std::ffi::CString;

extern {
    fn load_file(file_name: *const libc::c_char);
    fn pause_audio();
    fn stop_audio();
    fn set_volume(volume: libc::c_int);
    fn seek_audio(percent: libc::c_double);
    fn seek_audio_by_sec(sec: libc::c_int);
    fn get_time_length() -> libc::c_int;
    fn is_stopping() -> libc::c_int;
    fn get_current_time_pos() -> libc::c_double;
}

pub fn play(filename: String) {
    unsafe {
        let _path = CString::new(filename).unwrap();
        load_file(_path.as_ptr());
    }
}

pub fn pause() {
    unsafe {
        pause_audio();
    }
}

pub fn stop() {
    unsafe {
        stop_audio();
    }
}

pub fn set_volume_to(volume: i32) {
    unsafe {
        set_volume(volume);
    }
}

pub fn seek_by_percent(percent: f64) {
    unsafe {
        seek_audio(percent);
    }
}

pub fn seek_by_second(sec: i32) {
    unsafe {
        seek_audio_by_sec(sec);
    }
}


pub fn get_current_time_length() -> i32 {
    unsafe {
        let length = get_time_length();
        length
    }
}

pub fn get_current_time_position() -> f64 {

    unsafe {
        let pos = get_current_time_pos();
        pos
    }
}

pub fn stopping() -> bool {
    unsafe {
        let is_stopping = is_stopping();
        if is_stopping > 0 {
            return true;
        } else {
            return false;
        }
    }
}
