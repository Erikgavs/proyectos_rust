use freya::{prelude::*};

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut show_popup = use_signal(|| false);
    let mut value = use_signal(String::new);

    rsx!(

        rect {
            height: "100%",
            width: "100%",
            cross_align: "center",
            main_align: "center",

            if *show_popup.read() {
                Popup {
                    oncloserequest: move |_| show_popup.set(false),
                    label {
                        "New note!"
                    }
                    Input {
                        value,
                        onchange: move |e| {
                            value.set(e)
                        }
                    }
                    Button{
                        label{
                            "New note"
                        }
                    }
                }
            }
            Button{
                onpress: move |_| show_popup.set(true),
                label {
                    "New note"
                }
            }
        }
    )
}
