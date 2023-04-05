use crate::view;
use crate::BoltContext;
use crate::Msg;
use crate::Request;
use yew::{html, Html};

pub fn request(bctx: &mut BoltContext, request: Request, req_tab: u8) -> Html {
    let method = request.method.to_string();
    let link = bctx.link.as_ref().unwrap();

    // FIXME: method
    html! {
        <div class="req">
            <div class="requestbar">
                <div class="">
                    <select id="methodselect" class="methodselect pointer" onchange={link.callback(|_| Msg::MethodChanged)}>
                        <option value="get" selected={if method == "get" { true } else { false }}>{"GET"}</option>
                        <option value="post" selected={if method == "post" { true } else { false }}>{"POST"}</option>
                    </select>
                </div>

                <div>
                    <input id="urlinput" class="urlinput" type="text" value={request.url.clone()} placeholder="http://" onchange={link.callback(|_| Msg::UrlChanged)}/>
                </div>

                <button class="sendbtn pointer" type="button" onclick={link.callback(|_| Msg::SendPressed)}>{"Send"}</button>
            </div>

            <div class="reqtabs">
                <div id="req_body_tab" class=" tab pointer tabSelected" onclick={link.callback(|_| Msg::ReqBodyPressed)}>{"Body"}</div>
                <div id="req_params_tab" class=" tab pointer" onclick={link.callback(|_| Msg::ReqParamsPressed)}>{"Params"}</div>
                <div id="req_headers_tab" class=" tab pointer" onclick={link.callback(|_| Msg::ReqHeadersPressed)}>{"Headers"}</div>
            </div>

            <div class="tabcontent">
                if req_tab == 1 {
                    <textarea id="reqbody" class="reqbody" value={request.body.clone()} placeholder="Request body" onchange={link.callback(|_| Msg::BodyChanged)}>

                    </textarea>
                } else if req_tab == 2 {
                    <table>
                        <tr>
                            <th>{"Key"}</th>
                            <th>{"Value"}</th>
                        </tr>
                        { for request.params.iter().enumerate().map(|(index, header)| view::param::render_params(bctx, index, request.params.len(), &header[0], &header[1])) }
                    </table>
                } else if req_tab == 3 {
                    <table>
                        <tr>
                            <th>{"Header"}</th>
                            <th>{"Value"}</th>
                        </tr>
                        { for request.headers.iter().enumerate().map(|(index, header)| view::header::render_reqheader(bctx, index, request.headers.len(), &header[0], &header[1])) }
                    </table>
                }
            </div>
        </div>

    }
}
