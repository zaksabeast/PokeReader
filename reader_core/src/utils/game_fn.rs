#[macro_export]
macro_rules! game_fn {
  ($fn_name:ident($($param_name:ident: $param_type:ty),*) -> $return_type:ty = $address:expr) => {
      #[cfg(not(target_os = "horizon"))]
      pub fn $fn_name($($param_name: $param_type),*) -> $return_type {
          Default::default()
      }

      #[cfg(target_os = "horizon")]
      pub fn $fn_name($($param_name: $param_type),*) -> $return_type {
          type GameFn = extern "C" fn($($param_type),*) -> $return_type;
          let game_fn: GameFn = unsafe { core::mem::transmute($address) };
          game_fn($($param_name),*)
      }
  };
}

pub use game_fn;
