use rand;
use rand::distributions::{IndependentSample, Range};
use sfml::audio::{Sound, SoundSource};

pub fn filter_random(sound: &mut Sound) {
    sound.set_volume(Range::new(50., 100.).ind_sample(&mut rand::thread_rng()));
    sound.set_pitch(Range::new(1., 1.2).ind_sample(&mut rand::thread_rng()));
}
