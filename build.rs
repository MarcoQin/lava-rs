extern crate cc;

fn main() {
    cc::Build::new()
        .file("src/player.c")
        .static_flag(true)
        .compile("libplayer.a");
}
