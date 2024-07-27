mod general_structs;
mod colors;

#[allow(unused_imports)]
use general_structs::*;

use colors::*;

use leptos::*;
use leptos_router::*;
use leptos_mview::mview;
use leptos_use::{
  use_color_mode_with_options,
  ColorMode,
  UseColorModeOptions,
  UseColorModeReturn,
};

turf::style_sheet!("src/styles/global.scss");

#[component]
pub fn App() -> impl IntoView {

  let UseColorModeReturn {
    mode,
    set_mode,
    ..
  } = use_color_mode_with_options(
    UseColorModeOptions::default()
      .attribute("theme")
  );

  let handle = move || {
    if mode.get() == ColorMode::Dark {
      set_mode.set(ColorMode::Light);
    } else { set_mode.set(ColorMode::Dark); }
  };

  mview! {
    Router {
      header {
        button
          class={ClassName::BTN_THEME}
          on:click={move |_| {handle()}}
        {
          f["{:?}", mode.get()]
        }
      }
      main {
        SecAllClrs {}
      }
    }
  }
}
