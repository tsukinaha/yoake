use input::{event::keyboard::{KeyState, KeyboardEventTrait}, Event, Libinput, LibinputInterface};
use libc::{O_RDONLY, O_RDWR, O_WRONLY};
use once_cell::sync::Lazy;
use xkbcommon::xkb;
use std::{fs::{File, OpenOptions}, sync::{atomic::AtomicBool, Arc}};
use std::os::unix::{fs::OpenOptionsExt, io::OwnedFd};
use std::path::Path;

use flume::{unbounded, Receiver, Sender};

pub struct InputChannel {
    pub tx: Sender<String>,
    pub rx: Receiver<String>,
}

pub static INPUT_CHANNEL: Lazy<InputChannel> = Lazy::new(|| {
    let (tx, rx) = unbounded::<String>();

    InputChannel { tx, rx }
});


pub struct Interface;

impl LibinputInterface for Interface {
    fn open_restricted(&mut self, path: &Path, flags: i32) -> Result<OwnedFd, i32> {
        OpenOptions::new()
            .custom_flags(flags)
            .read((flags & O_RDONLY != 0) | (flags & O_RDWR != 0))
            .write((flags & O_WRONLY != 0) | (flags & O_RDWR != 0))
            .open(path)
            .map(|file| file.into())
            .map_err(|err| err.raw_os_error().unwrap())
    }
    fn close_restricted(&mut self, fd: OwnedFd) {
        drop(File::from(fd));
    }
}

impl Interface {
    pub fn run_event_loop(input_controll: Arc<AtomicBool>) -> anyhow::Result<()> {
        let mut input = Libinput::new_with_udev(Self);
        input.udev_assign_seat("seat0").map_err(|_| anyhow::anyhow!("Error assigning udev seat0"))?;
        
        let context = xkb::Context::new(xkb::CONTEXT_NO_FLAGS);
        let keymap = xkb::Keymap::new_from_names(
            &context,
            "", "", "", "", None,
            xkb::KEYMAP_COMPILE_NO_FLAGS
        ).unwrap();
        let mut state = xkb::State::new(&keymap);
        
        while input_controll.load(std::sync::atomic::Ordering::Relaxed) {
            input.dispatch()?;
            for event in &mut input {
                if let Event::Keyboard(key) = event {
                    let keycode = (key.key() + 8).into();
                    
                    state.update_key(
                        keycode,
                        if key.key_state() == KeyState::Released {
                            continue;
                        } else {
                            xkb::KeyDirection::Down
                        }
                    );

                    let keysym = state.key_get_one_sym(keycode);

                    if let Some(name) = keysym.name() {
                        INPUT_CHANNEL.tx.send(name[3..].to_owned()).unwrap();
                    } 
                }
            }
        };
        println!("Input loop ended");
        Ok(())
    }
}