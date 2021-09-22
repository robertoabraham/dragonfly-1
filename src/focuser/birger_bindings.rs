/* automatically generated by rust-bindgen 0.59.1 */

extern "C" {
    pub fn main(
        argc: ::std::os::raw::c_int,
        argv: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub static mut verbose: ::std::os::raw::c_int;
}
extern "C" {
    pub static mut help: [*mut ::std::os::raw::c_char; 0usize];
}
extern "C" {
    pub fn GetCurrentFocuserPosition(fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn FocuserPrintStatus(fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn FocuserPrintCurrentPosition(fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn FocuserMove(
        fd: ::std::os::raw::c_int,
        position: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn FocuserGoTo(
        fd: ::std::os::raw::c_int,
        position: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn FocuserInit(fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn FocuserSendRawCommand(
        fd: ::std::os::raw::c_int,
        command: *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn FocuserSetVerboseMode(fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn open_focuser_port(portname: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn value_from_focus_command_key(key: *mut ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}