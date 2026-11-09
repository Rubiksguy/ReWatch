//imports u dumb ahh
use mpv::MpvHandlerBuilder;
use std::env;

//Really Bad Horrible Terrible Code
fn main() {
    //collects all args in cli
    let files: Vec<String> = env::args().skip(1).collect();
    //makes it so that files needs atleast one file
    if files.is_empty() {
        eprintln!("usage: ReWatch <file> [file...]");
        return;
    }

    let mut builder = MpvHandlerBuilder::new().expect("mpv");
    //uses those weird looking on screen controls i really hate but im too lazy to make my own (and im too stoopid)
    let _ = builder.set_option("osc", "yes");
    //keyboard stuff (makes it so the default keyboard and mouse shortcuts from mpv are used)
    let _ = builder.set_option("input-default-bindings", "yes");
    //finalizes the mpv instance and smth else (im stackoverflow carried)
    let mut mpv = builder.build().expect("build");
    // Loads the first file immediately to start playback and appends(attaches to the end) other files you added (if you add multiple)
    mpv.command(&["loadfile", &files[0]]).expect("load");
    for f in &files[1..] {
        let _ = mpv.command(&["loadfile", f, "append"]);
    }

    loop {
        match mpv.wait_event(1.0) {
            //checks  if playback is idle or Shutdown
            Some(mpv::Event::Idle) | Some(mpv::Event::Shutdown) => break,
            _ => {}
        }
    }
}
