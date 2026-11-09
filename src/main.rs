use mpv::MpvHandlerBuilder;
use std::env;

fn main() {
    let path = env::args().nth(1).expect("usage: ReWatch <file>");
    let mut mpv = MpvHandlerBuilder::new().expect("mpv").build().expect("build");
    mpv.command(&["loadfile", &path]).expect("load");
    loop {
        match mpv.wait_event(1.0) {
            Some(mpv::Event::EndFile(_)) | Some(mpv::Event::Idle) | Some(mpv::Event::Shutdown) => break,
            _ => {}
        }
    }
}
