use yew::prelude::*;
use crate::domain::geometry::Size2d;
use crate::domain::interpreter::interpreter::Interpreter;
use crate::domain::parser::parser::Parser;
use crate::domain::turtle::Turtle;
use std::f64;
use crate::domain::config::{CANVAS_ID, INITIAL_CODE};

use wasm_bindgen::prelude::*;
use web_sys::HtmlTextAreaElement;
use yew::classes;
use yew::Properties;


fn get_canvas_size() -> Option<Size2d> {
    let canvas =
        web_sys::window()?
            .document()?
            .get_element_by_id(CANVAS_ID)?
            .dyn_into::<web_sys::HtmlCanvasElement>().ok()?;
    Some(Size2d {
        width: canvas.width() as f64,
        height: canvas.height() as f64,
    })
}

fn run_interpreter(code: &str) -> Result<String, String> {
    let canvas_size =
        &get_canvas_size()
        .expect("Could not get a canvas size. Is the canvas ready?");
    let turtle = Turtle::new_for_canvas(canvas_size);

    let code_block = Parser::new_from_str(code).parse();
    if code_block.is_err() {
        let error_message = code_block.err().unwrap().to_string();
        return Err(error_message);
    }
    let mut interpreter = Interpreter::new(turtle);
    match interpreter.interpret_script(&mut code_block.unwrap()){
        Ok(_) => Ok("✅ Done!".to_string()),
        Err(error) => Err(error.to_string())
    }
}


#[derive(Properties, PartialEq)]
pub struct CodeEditorElementProps {
    pub is_canvas_cover_visible_handle: UseStateHandle<bool>
}

#[function_component]
pub fn CodeEditorElement(props: &CodeEditorElementProps) -> Html {
    let code_value_handle = use_state(|| String::from(INITIAL_CODE));
    let code_value = (*code_value_handle).clone();
    let console_output_value_handle = use_state(String::default);
    let console_output_value = (*console_output_value_handle).clone();
    let last_key_pressed_is_ctrl_handle = use_state(|| false);
    let has_run_resulted_in_error_handle = use_state(|| false);

    let on_code_input = Callback::from({
        let code_value_handle = code_value_handle.clone();
        move |input_event: InputEvent| {
            let target: HtmlTextAreaElement = input_event
                .target()
                .unwrap_throw()
                .dyn_into()
                .unwrap_throw();
            code_value_handle.set(target.value());
        }
    });

    let on_run_button_click: Callback<MouseEvent> = {
        let has_run_resulted_in_error_handle = has_run_resulted_in_error_handle.clone();
        let input_value_handle = console_output_value_handle.clone();
        let is_canvas_cover_visible_handle = props.is_canvas_cover_visible_handle.clone();
        let code = code_value.clone();
        Callback::from(move |_| {
            match run_interpreter(code.as_str()) {
                Ok(message) => {
                    input_value_handle.set(message);
                    is_canvas_cover_visible_handle.set(false);
                    has_run_resulted_in_error_handle.set(false);
                },
                Err(message) => {
                    input_value_handle.set(message);
                    has_run_resulted_in_error_handle.set(true);
                }
            }
        })
    };
    let on_key_down_inside_code_editor: Callback<KeyboardEvent> = {
        let has_run_resulted_in_error_handle = has_run_resulted_in_error_handle.clone();
        let input_value_handle = console_output_value_handle.clone();
        let is_canvas_cover_visible_handle = props.is_canvas_cover_visible_handle.clone();
        let code_value = code_value.clone();
        let last_key_pressed_is_ctrl_handle = last_key_pressed_is_ctrl_handle.clone();
        let last_key_pressed_is_ctrl = (*last_key_pressed_is_ctrl_handle).clone();
        Callback::from(move |e:KeyboardEvent| {
            let should_run_interpreter = last_key_pressed_is_ctrl && e.code() == "Enter";
            last_key_pressed_is_ctrl_handle.set(e.ctrl_key() || e.meta_key());
            if should_run_interpreter {
                match run_interpreter(code_value.as_str()) {
                    Ok(message) => {
                        input_value_handle.set(message);
                        is_canvas_cover_visible_handle.set(false);
                        has_run_resulted_in_error_handle.set(false);
                    },
                    Err(message) => {
                        input_value_handle.set(message);
                        has_run_resulted_in_error_handle.set(true);
                    }
                }
            }
        })
    };

    html! {
        <div class="columns is-desktop">
            <div class="column">
                <div class="card box">
                    <div class="block is-flex is-justify-content-space-between">
                        <div class="">
                            <h2 class="is-size-5 icon-text">
                                <span class="icon">
                                    <ion-icon name="code-slash-outline"></ion-icon>
                                </span>
                                <span>{"Code"}</span>
                            </h2>
                        </div>
                        <div class="">
                            <button
                                class="button is-success is-small"
                                onclick={on_run_button_click}>
                                <span class="icon">
                                    <ion-icon name="play"></ion-icon>
                                </span>
                                <span>{ "Run (⌘ ↵)" }</span>
                            </button>
                        </div>
                    </div>
                    <div class="block body">
                        <textarea
                            class="textarea textarea-code"
                            spellcheck="false"
                            placeholder="Your code..."
                            onkeydown={on_key_down_inside_code_editor}
                            oninput={on_code_input}
                            value={code_value}></textarea>
                    </div>
                </div>
            </div>

            <div class="column">
                <div class="card box">
                    <div class="block header">
                        <div>
                            <h2 class="is-size-5 icon-text">
                                <span class="icon">
                                    <ion-icon name="terminal-outline"></ion-icon>
                                </span>
                                <span>{"Console output"}</span>
                            </h2>
                        </div>
                    </div>
                    <div class="block body">
                        <div class={
                            classes!(
                                "textarea",
                                "textarea-code",
                                (*has_run_resulted_in_error_handle).clone().then(|| Some("is-danger"))
                            )
                        }>
                            { console_output_value }
                        </div>
                    </div>
                </div>
            </div>
        </div>
    }
}