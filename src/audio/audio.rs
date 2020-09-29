use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    audio::{output::Output, OggFormat, Source, SourceHandle},
    ecs::{World, WorldExt},
};

pub struct Sounds {
    pub small_rock_sfx: SourceHandle,
    pub large_rock_sfx: SourceHandle,
    pub wrench_sfx: SourceHandle,
    pub item_sfx: SourceHandle,
    pub spaceship_laser_sfx: SourceHandle,
    pub crash_sfx: SourceHandle,
    pub explosion_sfx: SourceHandle,
    pub spaceship_hit_sfx: SourceHandle,
    pub cash_register_bell_sfx: SourceHandle,
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(file, OggFormat, (), &world.read_resource())
}

pub fn initialize_audio(world: &mut World) {
    let sound_effects = {
        let loader = world.read_resource::<Loader>();

        Sounds {
            small_rock_sfx: load_audio_track(&loader, &world, "audio/small_rock.ogg"),
            large_rock_sfx: load_audio_track(&loader, &world, "audio/large_rock.ogg"),
            wrench_sfx: load_audio_track(&loader, &world, "audio/wrench.ogg"),
            item_sfx: load_audio_track(&loader, &world, "audio/item.ogg"),
            spaceship_laser_sfx: load_audio_track(&loader, &world, "audio/spaceship_laser.ogg"),
            crash_sfx: load_audio_track(&loader, &world, "audio/crash.ogg"),
            explosion_sfx: load_audio_track(&loader, &world, "audio/explosion.ogg"),
            spaceship_hit_sfx: load_audio_track(&loader, &world, "audio/spaceship_hit.ogg"),
            cash_register_bell_sfx: load_audio_track(&loader, &world, "audio/cash_register_bell.ogg"),
        }
    };

    world.insert(sound_effects);
}

pub fn play_sfx(sound: &Handle<Source>, storage: &AssetStorage<Source>, output: Option<&Output>) {
    if let Some(ref output) = output.as_ref() {
        if let Some(sound) = storage.get(sound) {
            output.play_once(sound, 1.0);
        }
    }
}
