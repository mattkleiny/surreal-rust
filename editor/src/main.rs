//! The main editor binary for the Surreal Project.

fn main() {
  let window = &MainWindow::new().unwrap();
  let window_handle = window.as_weak().unwrap();

  window.on_say_hello(move || {
    let income = window_handle.get_total_income();

    println!("Looks like your total income is {}", income);
  });

  window.run().unwrap();
}

slint::slint! {
  import { LineEdit, Button, AboutSlint } from "std-widgets.slint";

  export component MainWindow inherits Window {
    title: "Income Calculator";
    background: #313e50;

    in-out property <int> total-income: 1000;

    callback say-hello();

    GridLayout {
      padding: 50px;
      spacing: 25px;

      Row {
        Text {
          text: "Enter Total Income";
          horizontal-alignment: center;
          font-size: 24px;
          font-weight: 900;
        }
      }

      Row {
        LineEdit {
          horizontal-alignment: center;
          font-size: 16px;
          placeholder-text: "Enter total income";
          text: total-income;
        }
      }

      Row {
        Button {
          text: "Calculate";
          primary: true;
          clicked => { say-hello() }
        }
      }

      AboutSlint {}
    }
  }
}
