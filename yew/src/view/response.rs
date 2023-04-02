use crate::view;
use crate::BoltApp;
use crate::Msg;
use crate::GLOBAL_STATE;
use yew::{html, Context, Html, AttrValue,};

pub fn response(ctx: &Context<BoltApp>) -> Html {
    let state = GLOBAL_STATE.lock().unwrap();

    html! {
    <div class="resp">
        <div class="respline">
            <div class="resptabs">
                <div id="resp_body_tab" class="tab pointer tabSelected" onclick={ctx.link().callback(|_| Msg::RespBodyPressed)}>{"Body"}</div>
                <div id="resp_headers_tab" class="tab pointer" onclick={ctx.link().callback(|_| Msg::RespHeadersPressed)}>{"Headers"}</div>
            </div>

            <div class="respstats">
                <div id="status" class="respstat">{"Status: "} {state.response.status}</div>
                <div id="time" class="respstat">{"Time: "} {state.response.time} {" ms"}</div>
                <div id="size" class="respstat">{"Size: "} {state.response.size} {" B"}</div>
            </div>
        </div>

        <div class="tabcontent">
            if state.resp_tab == 1 {
                <div id="respbody" class="respbody" >
                    {Html::from_html_unchecked(AttrValue::from(state.response.body.clone()))}
                </div>
            } else if state.resp_tab == 2 {
                <div class="respheaders">
                    <table>
                        <tr>
                            <th>{"Key"}</th>
                            <th>{"Value"}</th>
                        </tr>
                        { for state.response.headers.iter().map(|header| view::header::render_header(&header[0], &header[1])) }
                    </table>
                </div>
            }
        </div>
    </div>
    }
}
