use log::info;
use yew::prelude::*;
use crate::domain::config::CANVAS_ID;
use crate::views::navbar::NavbarElement;
use crate::views::code_editor::CodeEditorElement;
use crate::views::reference_card::ReferenceCardElement;


#[function_component(App)]
pub fn app() -> Html {
    let is_canvas_cover_visible_handle = use_state(|| true);
    info!("is_canvas_cover_visible_handle {}", (*is_canvas_cover_visible_handle).clone());
    html! {
        <>
            <NavbarElement />
            <main class="container">
                <div id="section__canvas"
                    class="section is-flex is-justify-content-space-around">
                    <div class="box">
                        <div class="wrapper">
                            if (*is_canvas_cover_visible_handle).clone() {
                                // I don't know how to recognize when canvas is ready to draw turtle.
                                // This is a workaround - hide the canvas behind a cover photo. :)
                                <img class="cover-image"
                                     src="/images/canvas-cover.png"
                                     width="365"
                                     height="365"/>
                            }
                            <canvas
                                id={CANVAS_ID}
                                class="container canvas-container"
                                height="365"
                                width="365">
                            </canvas>
                        </div>
                    </div>
                </div>
                <div class="section">
                    <CodeEditorElement
                        is_canvas_cover_visible_handle={is_canvas_cover_visible_handle.clone()}/>
                    <ReferenceCardElement />
                </div>
            </main>
            <footer class="section">
                <div class="container has-text-centered">
                    <p>{"Made using Rust, Yew, and WASM."}</p>
                    <p>{"Handcrafted by Miro Jelaska"}</p>
                    <p>
                        <a href="https://github.com/miro-jelaska/rusty-turtle"
                            target="_blank"
                            rel="noreferrer">
                            {"Star on GitHub"}
                        </a>
                    </p>
                </div>
            </footer>
        </>
    }
}
