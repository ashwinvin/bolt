use crate::view;
use crate::BoltContext;
use crate::Msg;
use crate::Page;
use crate::Request;
use yew::{html, Html};

pub fn request(bctx: &mut BoltContext) -> Html {
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

    let mut request = Request::new();

    if bctx.page == Page::Home && can_display {
        request = bctx.main_col.requests[bctx.main_current].clone();
    }

    if bctx.page == Page::Collections && can_display {
        request = bctx.collections[bctx.col_current[0]].requests[bctx.col_current[1]].clone();
    }

    let method = request.method.to_string();

    // FIXME: method
    html! {
        <div class="req">
        if can_display {
            <div class="requestbar">
                <div class="">
                    <select id="methodselect" class="methodselect pointer" onchange={link.callback(|_| Msg::MethodChanged)}>
                        <option value="get" selected={if method == "get" { true } else { false }}>{"GET"}</option>
                        <option value="post" selected={if method == "post" { true } else { false }}>{"POST"}</option>
                    </select>
                </div>

                <input id="urlinput" class="urlinput" type="text" value={request.url.clone()} placeholder="http://" onchange={link.callback(|_| Msg::UrlChanged)}/>

                <button class="sendbtn pointer" type="button" onclick={link.callback(|_| Msg::SendPressed)}>{"Send"}</button>
            </div>

            <div class="reqtabs">
                <div id="req_body_tab" class={if request.req_tab == 1  {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::ReqBodyPressed)}>{"Body"}</div>
                <div id="req_params_tab" class={if request.req_tab == 2  {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::ReqParamsPressed)}>{"Params"}</div>
                <div id="req_headers_tab" class={if request.req_tab == 3  {"tab pointer tabSelected"} else {"tab pointer"}} onclick={link.callback(|_| Msg::ReqHeadersPressed)}>{"Headers"}</div>
            </div>

            <div class="tabcontent">
                if request.req_tab == 1 {
                    <textarea id="reqbody" class="reqbody" value={request.body.clone()} placeholder="Request body" onchange={link.callback(|_| Msg::BodyChanged)}>

                    </textarea>
                } else if request.req_tab == 2 {
                    <table>
                        <tr>
                            <th>{"Key"}</th>
                            <th>{"Value"}</th>
                        </tr>
                        { for request.params.iter().enumerate().map(|(index, header)| view::param::render_params(bctx, index, request.params.len(), &header[0], &header[1])) }
                    </table>
                } else if request.req_tab == 3 {
                    <table>
                        <tr>
                            <th>{"Header"}</th>
                            <th>{"Value"}</th>
                        </tr>
                        { for request.headers.iter().enumerate().map(|(index, header)| view::header::render_reqheader(bctx, index, request.headers.len(), &header[0], &header[1])) }
                    </table>
                }
            </div>
        }
        </div>

    }
}
