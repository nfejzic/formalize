use alemat::{BufMathMlWriter, DisplayAttr, MathMl, MathMlAttr, Writer};
use leptos::{ev::Event, event_target_value, IntoView, SignalUpdate, WriteSignal};

fn render_math(input: &str) -> String {
    let ascii_math = mathemascii::parse(input);

    let mathml = MathMl::from(ascii_math)
        .with_attr([MathMlAttr::Display(DisplayAttr::Block)])
        .write(&mut BufMathMlWriter::default())
        .map(Writer::finish)
        .unwrap();

    mathml
}

fn set_mathml(text_setter: WriteSignal<String>, input: String) {
    let mathml = render_math(&input);
    text_setter.update(|t| *t = mathml)
}

#[leptos::component]
pub fn AsciiMathInput(initial: &'static str, text_setter: WriteSignal<String>) -> impl IntoView {
    let update_text = move |ev: Event| set_mathml(text_setter, event_target_value(&ev));

    // set the initial value
    set_mathml(text_setter, String::from(initial));

    leptos::view! {
        <div class="ascii-math-input">
            <textarea type="text" on:input=update_text>
                {initial}
            </textarea>
        </div>
    }
}
