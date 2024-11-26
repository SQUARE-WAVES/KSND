# KSND
a program in rust for editing sounds

# What is this for?
Ths is an old school style "two track edtior" in the vein of programs like SoundForge or OcenAudio. You open a sound file, or a few of them in different windows. 
then you can do things like copy and paste or delete bits of them.

The interface is built around using keyboard chords and script commands, kinda like the VIM text editor but for sounds.

# How do I build this?
you will need rust and cargo, Currently this is built using rust/cargo v 1.82.0.

For the most part this is a simple cargo build, *HOWEVER* at this point it depends on the development branch of the [iced](https://github.com/iced-rs/iced) crate. meaning you will have to
download iced v14.0-dev and build it locally, the current cargo.toml file assumes you have the iced repository cloned at the same root level as this project. 
After that you should just be able to type `cargo run` and go.

# This is like super alpha software right now!
I'm not including any real instructions about how to use this program yet, and I have barely tested it. 
If you want to play with it just understand that it's probably kinda buggy and the user interface is a bit weird.

So far I've only run it on an older Macbook pro, though in theory it should be windows and linux compatable. I just haven't tried them yet.
