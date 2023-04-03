use crate::view;
use crate::BoltApp;
use stylist::yew::Global;
use yew::{html, Context, Html};

pub fn collections_view(sel: &BoltApp, ctx: &Context<BoltApp>) -> Html {
    html! {
        <>
        <Global css={sel.style.clone()} />

        <body>
            {view::navbar::get_navbar(ctx)}

            <div class="main">
                {view::sidebar1::sidebar1(ctx, 1)}
                {view::sidebar2::sidebar2(ctx)}

                <div class="content">
                    // {view::request::request(ctx)}
                    {view::response::response(ctx)}
                </div>
            </div>

            // {view::console::console()}
        </body>
        </>
    }
}
