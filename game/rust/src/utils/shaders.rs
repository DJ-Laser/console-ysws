pub mod glitch;

macro_rules! shader_param_impls {
  ($class:ident { $($name:ident: $type:ty),* } ) => {
    paste::paste! {
      #[godot_api]
      impl $class {
        $(pub const [<$name:upper _PARAM>]: &str = stringify!($name);)*

        $(
          #[func]
          pub fn $name(&self) -> $type {
            self
            .material
            .get_shader_parameter(Self::[<$name:upper _PARAM>])
            .to()
          }

          #[func]
          pub fn [<set_ $name>](&mut self, value: $type) {
            self
            .material
            .set_shader_parameter(Self::[<$name:upper _PARAM>], &value.to_variant());
          }
        )*

        #[func]
        pub fn material(&self) -> Gd<godot::classes::Material> {
          self.material.clone().upcast()
        }
      }
    }
  };
}

use shader_param_impls;
