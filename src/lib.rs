use leptos::{create_signal, view, IntoView};

mod input;

#[leptos::component]
fn Header(level: u8, text: &'static str) -> impl IntoView {
    view! {
        {move || {
             return match level {
                2 => view!{<h2 class="header">{text}</h2>}.into_view(),
                3 => view!{<h3 class="header">{text}</h3>}.into_view(),
                4 => view!{<h4 class="header">{text}</h4>}.into_view(),
                5 => view!{<h5 class="header">{text}</h5>}.into_view(),
                6 => view!{<h6 class="header">{text}</h6>}.into_view(),
                _ => view!{<h1 class="header">{text}</h1>}.into_view(),
            }
        }
        }
    }
}

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
