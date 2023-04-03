use crate::view;
use crate::BoltApp;
use crate::Msg;
use crate::GLOBAL_STATE;
use crate::ResponseType;
use yew::{html, AttrValue, Context, Html};

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
                <div id="status" class="respstat">{"Status: "} {state.requests[state.current_request].response.status}</div>
                <div id="time" class="respstat">{"Time: "} {state.requests[state.current_request].response.time} {" ms"}</div>
                <div id="size" class="respstat">{"Size: "} {state.requests[state.current_request].response.size} {" B"}</div>
            </div>
        </div>

        <div class="tabcontent">
            if state.resp_tab == 1 {
                <div id="respbody" class="respbody" >
                    if state.requests[state.current_request].response.response_type == ResponseType::JSON {
                        {Html::from_html_unchecked(AttrValue::from(state.requests[state.current_request].response.body.clone()))}
                    } else {
                        {state.requests[state.current_request].response.body.clone()}
                    }
                </div>
            } else if state.resp_tab == 2 {
                <div class="respheaders">
                    <table>
                        <tr>
                            <th>{"Key"}</th>
                            <th>{"Value"}</th>
                        </tr>
                        { for state.requests[state.current_request].response.headers.iter().map(|header| view::header::render_header(&header[0], &header[1])) }
                    </table>
                </div>
            }
        </div>
    </div>
    }
}
