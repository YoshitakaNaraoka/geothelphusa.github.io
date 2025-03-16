use stylist::{css, Style};

pub fn center_styles() -> Style {
  Style::new(css!(
      r#"
          justify-content: center;
          text-align: center;
      "#
  ))
  .unwrap()
}

pub fn light_mode_styles() -> Style {
  Style::new(css!(
      r#"
          color: #0f0f0f;
          background-color: #f6f6f6;
          position: fixed;
          width: 100vw;
          height: 100vh;
      "#
  ))
  .unwrap()
}

pub fn dark_mode_styles() -> Style {
  Style::new(css!(
      r#"
          color: #f6f6f6;
          background-color: #2f2f2f;
          position: fixed;
          width: 100vw;
          height: 100vh;
      "#
  ))
  .unwrap()
}

pub fn base_styles() -> Style {
  Style::new(css!(
      r#"
          font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
          font-size: 16px;
          line-height: 24px;
          font-weight: 400;
          
          font-synthesis: none;
          text-rendering: optimizeLegibility;
          -webkit-font-smoothing: antialiased;
          -moz-osx-font-smoothing: grayscale;
          -webkit-text-size-adjust: 100%;
          text-align: center;
      "#
  ))
  .unwrap()
}

pub fn container_styles() -> Style {
  Style::new(css!(
      r#"
          margin: 0;
          
          padding-top: 10vh;
          display: flex;
          flex-direction: column;
          justify-content: top;
          text-align: center;
      "#
  ))
  .unwrap()
}

// pub fn row_styles() -> Style {
//   Style::new(css!(
//       r#"
//           display: flex;
//           justify-content: center;
//       "#
//   ))
//   .unwrap()
// }

// pub fn input_and_button() -> Style {
//   Style::new(css!(
//       r#"
//           border-radius: 8px;
//           border: 1px solid transparent;
//           padding: 0.6em 1.2em;
//           font-size: 1em;
//           font-weight: 500;
//           font-family: inherit;
//           color: #0f0f0f;
//           background-color: #ffffff;
//           transition: border-color 0.25s;
//           box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
//           outline: none;
//       "#
//   )).unwrap()
// }

// pub fn fit_picture() -> Style {
//   Style::new(css!(
//       r#"
//           width: 100%;
//           height: auto;
//       "#
//   )).unwrap()
// }

pub fn title_logo() -> Style {
  Style::new(css!(
      r#"
          width: 500px;
          height: 500px;
          max-width: 100%; /* 画面幅に応じてサイズを調整 */
          max-height: 100%;
      "#
  )).unwrap()
}

pub fn nav_styles() -> Style {
  Style::new(css!(
      r#"
          width: 100%;
          height: 70px;
          background-color: dimgray;
          padding-top: 5px;
          box-sizing: border-box;
          display: flex;
          list-style: none;
          max-width: 100%; /* 画面幅に応じてサイズを調整 */
          max-height: 100%;
      "#
  )).unwrap()
}

pub fn menu_items() -> Style {
  Style::new(css!(
      r#"
          display: block;
          text-decoration: none;
          color: white;
          margin-right: 35px;
      "#
  )).unwrap()
}

pub fn li_none() -> Style {
  Style::new(css!(
      r#"
          list-style: none;
      "#
  )).unwrap()
}

pub fn toggle_button() -> Style {
  Style::new(css!(
      r#"
            position: relative;
            display: flex;
            width: 56px;
            height: 28px;
            border: 1px solid #555555;
            border-radius: 9999px;
            background-color: #dddddd;
            cursor: pointer;

          :has(:focus-visible) {
            outline: auto;
            outline: auto -webkit-focus-ring-color;
          }

      "#
  )).unwrap()
}

pub fn toggle_slider() -> Style {
  Style::new(css!(
      r#"
          
            appearance: none;
            position: absolute;
            top: 40%;
            left: 0px;
            width: 24px;
            height: 24px;
            border: 1px solid #555555;
            border-radius: 9999px;
            transform: translateY(-50%);
            outline: none;
            background-color: #ffffff;
            transition: left 0.2s;
            cursor: pointer;
          

          :checked {
            left: calc(100% - 32px);
            background-color: #4ade80;
          }
      "#
  )).unwrap()
}

pub fn overlay_style() -> Style {
  Style::new(css!(
  r#"
        position: fixed;
        top: 0;
        left: 0;
        width: 100%;
        height: 100%;
        background: rgba(0, 0, 0, 0.5);
        display: flex;
        justify-content: center;
        align-items: center;
        z-index: 10;
        opacity: 0;
        visibility: hidden;
        transition: opacity 0.3s, visibility 0.3s;

        &.is-opened {
            opacity: 1;
            visibility: visible;
        }
  "#
))
.unwrap()
}

pub fn menu_style() -> Style {
  Style::new(css!(
  r#"
        background: white;
        padding: 20px;
        border-radius: 10px;
        box-shadow: 0 4px 10px rgba(0, 0, 0, 0.2);
        display: flex;
        flex-direction: column;
        row-gap: 10px;
        z-index: 20;
  "#
))
.unwrap()
}

pub fn button_style() -> Style {
  Style::new(css!(
  r#"
  width: 40px;
  height: 40px;
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: center;
  row-gap: 6px;
  background: white;
  border: 1px solid #333;
  border-radius: 4px;

  &__line,
  &::before,
  &::after {
      content: "";
      width: 28px;
      height: 2px;
      background-color: #333;
      transition: transform 0.3s, opacity 0.3s;
  }

  &.is-opened &__line {
      opacity: 0;
  }

  &.is-opened::before {
      transform: translateY(8px) rotate(45deg);
  }

  &.is-opened::after {
      transform: translateY(-8px) rotate(-45deg);
  }
  "#
))
.unwrap()
}

pub fn menu_list_style() -> Style {
  Style::new(css!(
      r#"
      list-style: none;
      padding: 0;
      margin: 0;
      display: flex;
      flex-direction: column;
      align-items: center;
      "#
  ))
  .unwrap()
}