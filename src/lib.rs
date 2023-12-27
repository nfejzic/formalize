use leptos::{create_signal, view, IntoView};

mod input;

#[leptos::component]
pub fn App() -> impl IntoView {
    let (text, update_text) = create_signal(String::default());

    view! {
        <h1 class="header">"Mathemascii - AsciiMath to mathml converter"</h1>

        <input::AsciiMathInput initial="a + b" text_setter=update_text />

        <h2 class="header mt">"Result:"</h2>

        <div class="rendered" inner_html={text}>
        </div>
    }
}
