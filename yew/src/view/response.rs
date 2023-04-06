use crate::Response;
use crate::view;
use crate::BoltContext;
use crate::Msg;
use crate::Page;
use crate::ResponseType;
use yew::{html, AttrValue, Html};

pub fn response(bctx: &mut BoltContext) -> Html {
    let resp_tab = bctx.resp_tab;
    let link = bctx.link.as_ref().unwrap();

    let mut can_display = false;

    if bctx.page == Page::Collections
        && bctx.collections.len() > 0
        && bctx.collections[bctx.col_current[0]].requests.len() > 0
    {
        can_display = true;
    }

    if bctx.page == Page::Home && bctx.main_col.requests.len() > 0 {
        can_display = true;
    }

    let mut response = Response::new();

    if bctx.page == Page::Home && can_display {
        response = bctx.main_col.requests[bctx.main_current].response.clone();
    }
    
    if bctx.page == Page::Collections && can_display {
        response = bctx.collections[bctx.col_current[0]].requests[bctx.col_current[1]]
            .response
            .clone();
    }

    html! {
    <div class="resp">
        if can_display {
        <div class="respline">
            <div class="resptabs">
                <div id="resp_body_tab" class="tab pointer tabSelected" onclick={link.callback(|_| Msg::RespBodyPressed)}>{"Body"}</div>
                <div id="resp_headers_tab" class="tab pointer" onclick={link.callback(|_| Msg::RespHeadersPressed)}>{"Headers"}</div>
            </div>

            <div class="respstats">
                <div id="status" class="respstat">{"Status: "} {response.status}</div>
                <div id="time" class="respstat">{"Time: "} {response.time} {" ms"}</div>
                <div id="size" class="respstat">{"Size: "} {response.size} {" B"}</div>
            </div>
        </div>

        <div class="tabcontent">
            if resp_tab == 1 {
                <div id="respbody" class="respbody" >
                    if response.response_type == ResponseType::JSON {
                        {Html::from_html_unchecked(AttrValue::from(response.body.clone()))}
                    } else {
                        {response.body.clone()}
                    }
                </div>
            } else if resp_tab == 2 {
                <div class="respheaders">
                    <table>
                        <tr>
                            <th>{"Key"}</th>
                            <th>{"Value"}</th>
                        </tr>
                        { for response.headers.iter().map(|header| view::header::render_header(&header[0], &header[1])) }
                    </table>
                </div>
            }
        </div>
        }
    </div>
    }
}
