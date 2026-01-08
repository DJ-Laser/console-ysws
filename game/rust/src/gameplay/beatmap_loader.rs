use godot::prelude::*;

use crate::{
  gameplay::{held_note::HeldNote, reticle::Reticle, single_note::SingleNote},
  rhythm::{
    note_manager::{DynNote, NoteManager},
    notes::RhythmInput,
    song::Beatmap,
  },
};

#[derive(Debug, GodotClass)]
#[class(init, base = Node2D)]
pub struct BeatmapLoader {
  #[export]
  beatmap: OnEditor<Gd<Beatmap>>,

  #[export]
  high_hit_location: OnEditor<Gd<Reticle>>,
  #[export]
  low_hit_location: OnEditor<Gd<Reticle>>,

  #[init(load = "res://objects/single_note/single_note.tscn")]
  single_note_scene: OnReady<Gd<PackedScene>>,

  #[init(load = "res://objects/held_note/held_note.tscn")]
  held_note_scene: OnReady<Gd<PackedScene>>,

  base: Base<Node2D>,
}

#[godot_api]
impl BeatmapLoader {
  pub fn load_beatmap_sprites(
    &mut self,
    note_manager: Gd<NoteManager>,
    mut add_note: impl FnMut(DynNote),
  ) {
    let mut input = RhythmInput::High;
    for i in 1..=20 {
      let note = self
        .single_note_scene
        .instantiate()
        .expect("Scene should be valid");
      let mut note: Gd<SingleNote> = note.cast();

      note.bind_mut().set_hit_beat(i as f64 * 1.0);
      note.bind_mut().set_note_manager(Some(note_manager.clone()));
      note.bind_mut().set_rhythm_input(input.to_godot());

      input = match input {
        RhythmInput::High => RhythmInput::Low,
        RhythmInput::Low => RhythmInput::High,
      };

      self.base_mut().add_child(&note);
      add_note(note.into_dyn().upcast());
    }

    let note = self
      .held_note_scene
      .instantiate()
      .expect("Scene should be valid");
    let mut note: Gd<HeldNote> = note.cast();

    note.bind_mut().set_start_beat(24.0);
    note.bind_mut().set_release_beat(30.0);
    note.bind_mut().set_note_manager(Some(note_manager.clone()));

    self.base_mut().add_child(&note);
    add_note(note.into_dyn().upcast());
  }

  /// Get the position of the hit location for the given input track
  pub fn get_hit_location(&self, input_track: RhythmInput) -> Vector2 {
    let global_pos = match input_track {
      RhythmInput::High => self.high_hit_location.get_global_position(),
      RhythmInput::Low => self.low_hit_location.get_global_position(),
    };

    self.base().to_local(global_pos)
  }
}
