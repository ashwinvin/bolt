use crate::view;
use crate::BoltContext;
use crate::Msg;
use crate::ResponseType;
use yew::{html, AttrValue, Html};

pub fn response(bctx: &mut BoltContext) -> Html {
    // let state = GLOBAL_STATE.lock().unwrap();
    let main_current = bctx.main_current;
    let resp_tab = bctx.resp_tab;
    let link = bctx.link.as_ref().unwrap();

    html! {
    <div class="resp">
        <div class="respline">
            <div class="resptabs">
                <div id="resp_body_tab" class="tab pointer tabSelected" onclick={link.callback(|_| Msg::RespBodyPressed)}>{"Body"}</div>
                <div id="resp_headers_tab" class="tab pointer" onclick={link.callback(|_| Msg::RespHeadersPressed)}>{"Headers"}</div>
            </div>

            <div class="respstats">
                <div id="status" class="respstat">{"Status: "} {bctx.main_col.requests[main_current].response.status}</div>
                <div id="time" class="respstat">{"Time: "} {bctx.main_col.requests[main_current].response.time} {" ms"}</div>
                <div id="size" class="respstat">{"Size: "} {bctx.main_col.requests[main_current].response.size} {" B"}</div>
            </div>
        </div>

        <div class="tabcontent">
            if resp_tab == 1 {
                <div id="respbody" class="respbody" >
                    if bctx.main_col.requests[main_current].response.response_type == ResponseType::JSON {
                        {Html::from_html_unchecked(AttrValue::from(bctx.main_col.requests[main_current].response.body.clone()))}
                    } else {
                        {bctx.main_col.requests[main_current].response.body.clone()}
                    }
                </div>
            } else if resp_tab == 2 {
                <div class="respheaders">
                    <table>
                        <tr>
                            <th>{"Key"}</th>
                            <th>{"Value"}</th>
                        </tr>
                        { for bctx.main_col.requests[main_current].response.headers.iter().map(|header| view::header::render_header(&header[0], &header[1])) }
                    </table>
                </div>
            }
        </div>
    </div>
    }
}
