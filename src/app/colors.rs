
use leptos::*;
use leptos_mview::mview;


turf::style_sheet!("src/styles/global.scss");

/*
let hss = vec![
  "surface", "aux", "mark", "comment",
  "active", "effect", "info", "success",
  "warning", "error", "alert", "focus",
  "main", "second", "detail", "default",
];
*/

#[component]
pub fn BoxClr(
  #[prop(default = String::from("main"))]
  clr_hs: String,
  #[prop(default = true)]
  named: bool,
  #[prop(default = String::from("32px"))]
  sized: String,
) -> impl IntoView {

  let ls = vec![
    "xf", "xe", "xd", "xc", "xb", "xa", "x9", "x8",
    "x7", "x6", "x5", "x4", "x3", "x2", "x1", "x0",
    "00",
    "y0", "y1", "y2", "y3", "y4", "y5", "y6", "y7",
    "y8", "y9", "ya", "yb", "yc", "yd", "ye", "yf"
  ];

  mview!{
    div
      class={ClassName::SEC_CLRS}
      style={format!("--sized-box-clr: {sized}; --clr-hs: var(--hs-{clr_hs});")}
    {
      div class={ClassName::TITLE}
        style={format!("grid-area: 1 / 1 / 1 / -1;")}
        { {&clr_hs} }
      {
        ls.into_iter().map(|v| {
          let sty = format!("--clr-l: var(--l-{v});");
          let mut style = format!("{sty}");
          if v == "00" {
            style = format!("{sty} grid-area: var(--sty-area-row) / 1 / var(--sty-area-row) / -1;");
          }
          mview! {
            div class={ClassName::BOX_CLR} {style} { { if named {v} else {""} } }
          }
        }).collect::<Vec<_>>()
      }
    }

  }
}

#[component]
pub fn SecAllClrs(

) -> impl IntoView {

  // let hss = vec![
  //   "surface", "aux", "mark", "comment",
  //   "active", "effect", "info", "success",
  //   "warning", "error", "alert", "focus",
  //   "main", "second", "detail", "default",
  // ];

  mview!(
    BoxClr clr-hs={"surface".to_string()} {}
    div class={ClassName::SEC_ALL_CLRS} {
      BoxClr clr-hs={"main".to_string()}    named=false sized={"24px".to_string()} {}
      BoxClr clr-hs={"mark".to_string()}    named=false sized={"24px".to_string()} {}
      BoxClr clr-hs={"focus".to_string()}   named=false sized={"24px".to_string()} {}
      BoxClr clr-hs={"second".to_string()}  named=false sized={"24px".to_string()} {}
      BoxClr clr-hs={"aux".to_string()}     named=false sized={"24px".to_string()} {}
      BoxClr clr-hs={"info".to_string()}    named=false sized={"24px".to_string()} {}
      BoxClr clr-hs={"comment".to_string()} named=false sized={"24px".to_string()} {}
      BoxClr clr-hs={"active".to_string()}  named=false sized={"24px".to_string()} {}
      BoxClr clr-hs={"effect".to_string()}  named=false sized={"24px".to_string()} {}
      BoxClr clr-hs={"success".to_string()} named=false sized={"24px".to_string()} {}
      BoxClr clr-hs={"warning".to_string()} named=false sized={"24px".to_string()} {}
      BoxClr clr-hs={"error".to_string()}   named=false sized={"24px".to_string()} {}
      BoxClr clr-hs={"alert".to_string()}   named=false sized={"24px".to_string()} {}
      BoxClr clr-hs={"detail".to_string()}  named=false sized={"24px".to_string()} {}
      BoxClr clr-hs={"default".to_string()} named=false sized={"24px".to_string()} {}
    }
  )
}
