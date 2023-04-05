use crate::BoltContext;
use crate::view;
use stylist::yew::Global;
use yew::{html, Html};
use crate::Request;

pub fn collections_view(bctx: &mut BoltContext, request: Request) -> Html {
    let req_tab = bctx.req_tab;
    // let ctx = bctx.ctx.unwrap();
    
    html! {
        <>
        <Global css={bctx.style.clone()} />

        <body>
            {view::navbar::get_navbar(bctx)}

            <div class="main">
                {view::sidebar1::sidebar(bctx, 1)}
                {view::sidebar2::sidebar_col(bctx)}

                <div class="content">
                    {view::request::request(bctx, request, req_tab)}
                    {view::response::response(bctx)}
                </div>
            </div>

            // {view::console::console()}
        </body>
        </>
    }
}
