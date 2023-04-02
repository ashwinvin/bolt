use yew::{html, Context, Html};
use crate::Msg;
use crate::BoltApp;
use crate::GLOBAL_STATE;
use crate::Request;

pub fn sidebar2(ctx: &Context<BoltApp>) -> Html {
    let state = GLOBAL_STATE.lock().unwrap();
    
    html!{
        <div class="sidebar2">
            <div>
                <div class="pointer" onclick={ctx.link().callback(|_| Msg::AddRequest)}>
                    <svg viewBox="0 0 1024 1024" fill="currentColor" height="20px" width="20px" ><defs><style /></defs><path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z" /><path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z" /></svg>
                </div>
            </div>

            { for state.requests.iter().enumerate().map(|(index, req)| render_request(ctx, index, state.current_request, req))}
 
        </div>
    }
}

fn render_request(ctx: &Context<BoltApp>, index: usize, selected: usize, req: &Request) -> Html {
    html! {
        if index == selected {
            <div id={"request".to_string() + &index.to_string()} class="sidebar2item sidebar2item-selected pointer" onclick={ctx.link().callback(move |_| Msg::SelectRequest(index))}>{req.name.clone() + &index.to_string()}</div>
        } else {
            <div id={"request".to_string() + &index.to_string()} class="sidebar2item pointer" onclick={ctx.link().callback(move |_| Msg::SelectRequest(index))}>{req.name.clone() + &index.to_string()}</div>
        }
    }
}