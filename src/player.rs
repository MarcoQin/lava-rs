extern crate libc;
use std::ffi::CString;

extern {
    fn global_init();
    fn cp_load_file(file_name: *const libc::c_char);
    fn cp_pause_audio();
    fn cp_stop_audio();
    fn cp_set_volume(volume: libc::c_int);
    fn cp_seek_audio(percent: libc::c_double);
    fn cp_seek_audio_by_sec(sec: libc::c_int);
    fn cp_seek_audio_by_absolute_pos(pos: libc::c_int);
    fn cp_get_time_length() -> libc::c_int;
    fn cp_is_stopping() -> libc::c_int;
    fn cp_get_current_time_pos() -> libc::c_double;
}

pub fn init_player() {
    unsafe {
        global_init();
    }
}

pub fn load_file(filename: String) {
    unsafe {
        let _path = CString::new(filename).unwrap();
        cp_load_file(_path.as_ptr());
    }
}

pub fn pause() {
    unsafe {
        cp_pause_audio();
    }
}

pub fn stop() {
    unsafe {
        cp_stop_audio();
    }
}

pub fn set_volume(volume: i32) {
    unsafe {
        cp_set_volume(volume);
    }
}

pub fn seek_by_percent(percent: f64) {
    unsafe {
        cp_seek_audio(percent);
    }
}

pub fn seek_by_second(sec: i32) {
    unsafe {
        cp_seek_audio_by_sec(sec);
    }
}

pub fn seek_by_position(pos: i32) {
    unsafe {
        cp_seek_audio_by_absolute_pos(pos);
    }
}

pub fn get_current_time_length() -> i32 {
    unsafe {
        let length = cp_get_time_length();
        length
    }
}

pub fn get_current_time_position() -> f64 {

    unsafe {
        let pos = cp_get_current_time_pos();
        pos
    }
}

pub fn is_stopping() -> bool {
    unsafe {
        let is_stopping = cp_is_stopping();
        if is_stopping > 0 {
            return true;
        } else {
            return false;
        }
    }
}