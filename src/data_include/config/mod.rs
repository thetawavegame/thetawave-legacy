use amethyst::utils::application_root_dir;
use std::fs::{DirBuilder, File};
use std::io::prelude::*;

macro_rules! confgen {
    ( $($filename:expr),* ) => {
        {
            let conf_dir = application_root_dir().unwrap().join("config");
            if !conf_dir.is_dir() {
                DirBuilder::new()
                    .create(conf_dir.clone())
                    .expect("Confgen failed: could not create config dir.");
            }

            $({
                let default = include_bytes!($filename);
                let file_path = conf_dir.join($filename);
                if !file_path.is_file() {
                    let mut file = File::create(file_path)
                        .expect(concat!("Confgen failed: could not create config file ", $filename, "."));
                    file.write_all(default)
                        .expect(concat!("Confgen failed: could not write config file ", $filename, "."));
                }
            })*
        }
    }
}

pub fn generate_configs() {
    confgen!(
        "bindings_config.ron",
        "debug_lines_config.ron",
        "display_config_640.ron",
        "display_config_960.ron",
        "display_fullscreen.ron",
        "sounds_config.ron",
        "spritesheets_config.ron"
    );
}
