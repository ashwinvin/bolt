use gloo::console;
use yew::{html, Component, Context, Html};

// Define the possible messages which can be sent to the component
pub enum Msg {
    Increment,
    Decrement,
}

pub struct App {
    value: i64, // This will store the counter value
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Increment => {
                self.value += 1;
                console::log!("plus one"); // Will output a string to the browser console
                true // Return true to cause the displayed change to update
            }
            Msg::Decrement => {
                self.value -= 1;
                console::log!("minus one");
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
        <body>

            <div class="navbar">
                <div class="logo">
                    <div>{"Bolt API"}</div>
                </div>

                <div class="nav-links">
                    <div>{"Docs"}</div>
                    <div>{"Settings"}</div>
                </div>
            </div>

            <div class="main">
                <div class="sidebar1">
                    <div>{"APIs"}</div>
                    <div>{"Collection"}</div>
                    <div>{"Test"}</div>
                </div>

                <div class="sidebar2">
                    <div>{"API 1"}</div>
                    <div>{"API 2"}</div>
                    <div>{"API 3"}</div>
                </div>

                <div class="content">
                    <h1>{"Welcome to Bolt"}</h1>
                    <p>{"Glad you're here!"}</p>
                </div>
            </div>

            <div class="footer">
                <p> {"Console"} </p>
            </div>

            <script src="script.js"></script>
        </body>




                         }
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
