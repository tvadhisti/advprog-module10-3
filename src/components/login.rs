use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("no context found");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="bg-gradient-to-r from-blue-500 to-purple-600 flex w-screen h-screen items-center justify-center">
            <div class="bg-white bg-opacity-50 p-6 rounded shadow-md text-center"> 
                <h2>{"Login"}</h2>
                <input type="text"
                       class="mt-4 p-2 border rounded focus:outline-none focus:border-blue-300"
                       placeholder="Username"
                       oninput={oninput}/>
                <Link<Route> to={Route::Chat}>
                <div>
                    <button {onclick}
                            disabled={username.len() < 1}
                            class="mt-4 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded disabled:bg-blue-300">
                        {"Login"}
                    </button>
                </div>
                </Link<Route>>
            </div>
        </div>
    }
}
