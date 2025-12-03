use freya::{prelude::*};

fn main() {
    launch(app);
}

fn app() -> Element {
    let mut show_popup = use_signal(|| false); // use_signal =  mades a reactive variable, ready for makeing animations

    rsx!(

        rect {
            height: "100%",
            width: "100%",
            main_align: "center",
            cross_align: "center",

            if *show_popup.read() { // if (if popup = true)|||||| *show_popup.read() => gets the value of show_popup = false, if it's true the popup starts
                Popup {
                    oncloserequest: move |_| show_popup.set(false), // when the popup is closed, returns the value to false
                    label {
                        "Popup on top!"
                    }
                }
            }
            Button {
                onpress: move |_| show_popup.set(true), // When we press the button, the state of the popup is true so the popu starts
                label {
                    "Try this pop-up!"
                }
            }
        }
    )
}
