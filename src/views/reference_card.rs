use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum Tab {
    Reference = 1,
    Examples = 2
}

#[function_component]
fn ReferenceElement() -> Html {
    html! {
        <>
        <div class="notification is-info is-light mt-2">
            {"Rusty Turtle supports only few essential LOGO commands."}
        </div>
        <table class="table">
            <thead>
            <tr>
                <th>{"Command"}</th>
                <th>{"Example"}</th>
                <th>{"Description"}</th>
            </tr>
            </thead>
            <tbody>
                <tr>
                    <td>{"FORWARD <number>"}<br/>{"FD <number>"}</td>
                    <td>{"FD 50"}</td>
                    <td>{"Move forward for <number>."}</td>
                </tr>

                <tr>
                    <td>{"BACK <number>"}<br/>{"BK <number>"}</td>
                    <td>{"BK 100"}</td>
                    <td>{"Move back for <number>."}</td>
                </tr>

                <tr>
                    <td>{"RIGHT <number>"}<br/>{"RT <number>"}</td>
                    <td>{"RT 60"}</td>
                    <td>{"Rotate to the right for <number> degrees."}</td>
                </tr>

                <tr>
                    <td>{"LEFT <number>"}<br/>{"LF <number>"}</td>
                    <td>{"LF 30"}</td>
                    <td>{"Rotate to the right for <number> degrees."}</td>
                </tr>

                <tr>
                    <td>{"COLOR <hex color>"}</td>
                    <td>{"COLOR #663399"}</td>
                    <td>{"Set color for the line."}</td>
                </tr>
                <tr>
                    <td>{"REPEAT <number> [ <code> ] "}</td>
                    <td>{"REPEAT 2 [ FD 50 RT 30 ]"}</td>
                    <td>{"<code> gets repeated <number> of times."}</td>
                </tr>
            </tbody>
        </table>
        </>
    }
}

#[function_component]
fn ExamplesElement() -> Html {
    html! {
        <div>
            <div class="card block">
                <div class="card-header">
                    <div class="card-header-title">{"Welcome example"}</div>
                </div>
                <div class="card-content">
                    <div class="code">
{"REPEAT 3 [
    COLOR #00ff00
    RT 60 FD 50
    COLOR #ff0000
    RT 60 FD 50
]"}
                    </div>
                </div>
            </div>
            <div class="card block">
                <div class="card-header">
                    <div class="card-header-title">{"Hexagon"}</div>
                </div>
                <div class="card-content">
                    <div class="code">
                        {"REPEAT 6 [ RT 60 FD 100 ]"}
                    </div>
                </div>
            </div>
        </div>
    }
}
#[function_component]
pub fn ReferenceCardElement() -> Html {
    let active_tab_handle = use_state(|| Tab::Reference);
    let active_tab = (*active_tab_handle).clone();

    let on_reference_click = {
        let active_tab = active_tab_handle.clone();
        Callback::from(move |_| {
            active_tab.set(Tab::Reference);
        })
    };

    let on_examples_click = {
        let active_tab = active_tab_handle.clone();
        Callback::from(move |_| {
            active_tab.set(Tab::Examples);
        })
    };

    html! {
        <div class="card">
            <div class="box">
                <div class="block">
                    <div class="tabs is-boxed">
                      <ul>
                        <li
                            class={(active_tab == Tab::Reference).then(|| Some("is-active"))}
                            onclick={on_reference_click}>
                            <a class="icon-text">
                                <span class="icon is-small"><ion-icon name="book-outline"></ion-icon></span>
                                <span>{"Reference"}</span>
                            </a>
                        </li>
                        <li
                            class={(active_tab == Tab::Examples).then(|| Some("is-active"))}
                            onclick={on_examples_click}>
                            <a class="icon-text">
                                <span class="icon is-small"><ion-icon name="copy-outline"></ion-icon></span>
                                <span>{"Examples"}</span>
                            </a>
                        </li>
                      </ul>
                    </div>
                    if active_tab == Tab::Reference {
                        <ReferenceElement/>
                    }
                    if active_tab == Tab::Examples {
                        <ExamplesElement />
                    }
                </div>
            </div>
        </div>
    }
}