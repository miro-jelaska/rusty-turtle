use yew::prelude::*;

#[function_component]
pub fn NavbarElement() -> Html {
    html! {
        <nav class="navbar is-black" role="navigation" aria-label="main navigation">
            <div class="container">
                <div class="navbar-brand">
                    <a class="navbar-item"
                        href="https://github.com/miro-jelaska/rusty-turtle"
                        target="_blank">
                        <img src="images/logo.png?v=" height="50"/>
                    </a>
                </div>

                <div id="navbarBasicExample" class="navbar-menu">
                    <div class="navbar-start"></div>

                    <div class="navbar-end">
                        <div class="navbar-item">
                            <div class="buttons">
                                <a href="https://github.com/miro-jelaska/rusty-turtle"
                                    target="_blank"
                                    class="button is-light is-small">
                                    <span class="icon">
                                        <ion-icon name="logo-github"></ion-icon>
                                    </span>
                                    <span>{"GitHub"}</span>
                                </a>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </nav>
    }
}