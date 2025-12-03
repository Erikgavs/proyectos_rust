use freya::{elements::label, prelude::*};

fn main() {
    launch(app)
}

fn app() -> Element {
    let mut value = use_signal(String::new);

    rsx!(
        rect {
            width: "100%",
            height: "100%",
            main_align: "center",
            cross_align: "center",

            label{
                "Put your name!"
            }

            label{
                "Name: {value}"
            }

            rect {
                spacing: "15",
                cross_align: "center",

                Input{
                    value,
                    onchange: move |e| {
                        value.set(e)
                    }
                }


                FilledButton{
                    onpress: move |_| {
                        let input_value = value.read().clone();
                        value.set(String::new());
                        println!("Datos guardados y enviados! {}", input_value)
                    },
                    label {
                        "click this!"
                    }
                }
            }
        }
    )
}
