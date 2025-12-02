use freya::{prelude::*};

fn main(){
    launch(app)
}

fn app() -> Element { // Element => content that we pass to freya to put it in the window
    rsx!( // macro from freya that allows the user to write html like code (just similar)
        rect { // rect = div
            width: "100%",
            height: "100%",
            main_align: "center",
            cross_align: "center",


            label { // => the same as a <p> o <span>
                font_size: "40",
                "Hello Word!"
            }

            label {
                font_size: "20",
                "By Erik!"
            }
        }
    )
}
