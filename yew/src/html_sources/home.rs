use crate::Msg;
use stylist::yew::Global;
use yew::{html, Context, Html};

use crate::BoltApp;

pub fn get_main(sel: &BoltApp, ctx: &Context<BoltApp>) -> Html {
    return html! {
        <>
        <Global css={sel.style.clone()} />

        <body>
            <div class="navbar">
                <div class="logo">
                    {"BOLT"}
                </div>

                <div class="nav-links">
                    <div class="helpicon pointer">
                        <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 24 24" height="25px" width="25px" xmlns="http://www.w3.org/2000/svg"><path d="M12 6a3.939 3.939 0 0 0-3.934 3.934h2C10.066 8.867 10.934 8 12 8s1.934.867 1.934 1.934c0 .598-.481 1.032-1.216 1.626a9.208 9.208 0 0 0-.691.599c-.998.997-1.027 2.056-1.027 2.174V15h2l-.001-.633c.001-.016.033-.386.441-.793.15-.15.339-.3.535-.458.779-.631 1.958-1.584 1.958-3.182A3.937 3.937 0 0 0 12 6zm-1 10h2v2h-2z"></path><path d="M12 2C6.486 2 2 6.486 2 12s4.486 10 10 10 10-4.486 10-10S17.514 2 12 2zm0 18c-4.411 0-8-3.589-8-8s3.589-8 8-8 8 3.589 8 8-3.589 8-8 8z"></path></svg>
                    </div>

                    <div class="settingsicon pointer">
                        <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 1024 1024" height="25px" width="25px" xmlns="http://www.w3.org/2000/svg"><path d="M512.5 390.6c-29.9 0-57.9 11.6-79.1 32.8-21.1 21.2-32.8 49.2-32.8 79.1 0 29.9 11.7 57.9 32.8 79.1 21.2 21.1 49.2 32.8 79.1 32.8 29.9 0 57.9-11.7 79.1-32.8 21.1-21.2 32.8-49.2 32.8-79.1 0-29.9-11.7-57.9-32.8-79.1a110.96 110.96 0 0 0-79.1-32.8zm412.3 235.5l-65.4-55.9c3.1-19 4.7-38.4 4.7-57.7s-1.6-38.8-4.7-57.7l65.4-55.9a32.03 32.03 0 0 0 9.3-35.2l-.9-2.6a442.5 442.5 0 0 0-79.6-137.7l-1.8-2.1a32.12 32.12 0 0 0-35.1-9.5l-81.2 28.9c-30-24.6-63.4-44-99.6-57.5l-15.7-84.9a32.05 32.05 0 0 0-25.8-25.7l-2.7-.5c-52-9.4-106.8-9.4-158.8 0l-2.7.5a32.05 32.05 0 0 0-25.8 25.7l-15.8 85.3a353.44 353.44 0 0 0-98.9 57.3l-81.8-29.1a32 32 0 0 0-35.1 9.5l-1.8 2.1a445.93 445.93 0 0 0-79.6 137.7l-.9 2.6c-4.5 12.5-.8 26.5 9.3 35.2l66.2 56.5c-3.1 18.8-4.6 38-4.6 57 0 19.2 1.5 38.4 4.6 57l-66 56.5a32.03 32.03 0 0 0-9.3 35.2l.9 2.6c18.1 50.3 44.8 96.8 79.6 137.7l1.8 2.1a32.12 32.12 0 0 0 35.1 9.5l81.8-29.1c29.8 24.5 63 43.9 98.9 57.3l15.8 85.3a32.05 32.05 0 0 0 25.8 25.7l2.7.5a448.27 448.27 0 0 0 158.8 0l2.7-.5a32.05 32.05 0 0 0 25.8-25.7l15.7-84.9c36.2-13.6 69.6-32.9 99.6-57.5l81.2 28.9a32 32 0 0 0 35.1-9.5l1.8-2.1c34.8-41.1 61.5-87.4 79.6-137.7l.9-2.6c4.3-12.4.6-26.3-9.5-35zm-412.3 52.2c-97.1 0-175.8-78.7-175.8-175.8s78.7-175.8 175.8-175.8 175.8 78.7 175.8 175.8-78.7 175.8-175.8 175.8z"></path></svg>
                    </div>
                </div>
            </div>

            <div class="main">
                <div class="sidebar1">
                    <div class="sidebaritem sidebaritem-selected pointer">
                        <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 1024 1024" height="25px" width="25px" xmlns="http://www.w3.org/2000/svg"><path d="M917.7 148.8l-42.4-42.4c-1.6-1.6-3.6-2.3-5.7-2.3s-4.1.8-5.7 2.3l-76.1 76.1a199.27 199.27 0 0 0-112.1-34.3c-51.2 0-102.4 19.5-141.5 58.6L432.3 308.7a8.03 8.03 0 0 0 0 11.3L704 591.7c1.6 1.6 3.6 2.3 5.7 2.3 2 0 4.1-.8 5.7-2.3l101.9-101.9c68.9-69 77-175.7 24.3-253.5l76.1-76.1c3.1-3.2 3.1-8.3 0-11.4zM578.9 546.7a8.03 8.03 0 0 0-11.3 0L501 613.3 410.7 523l66.7-66.7c3.1-3.1 3.1-8.2 0-11.3L441 408.6a8.03 8.03 0 0 0-11.3 0L363 475.3l-43-43a7.85 7.85 0 0 0-5.7-2.3c-2 0-4.1.8-5.7 2.3L206.8 534.2c-68.9 68.9-77 175.7-24.3 253.5l-76.1 76.1a8.03 8.03 0 0 0 0 11.3l42.4 42.4c1.6 1.6 3.6 2.3 5.7 2.3s4.1-.8 5.7-2.3l76.1-76.1c33.7 22.9 72.9 34.3 112.1 34.3 51.2 0 102.4-19.5 141.5-58.6l101.9-101.9c3.1-3.1 3.1-8.2 0-11.3l-43-43 66.7-66.7c3.1-3.1 3.1-8.2 0-11.3l-36.6-36.2z"></path></svg>
                        {"Requests"}
                    </div>

                    <div class="sidebaritem pointer">
                        <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 16 16" height="25px" width="25px" xmlns="http://www.w3.org/2000/svg"><path d="M0 13a1.5 1.5 0 0 0 1.5 1.5h13A1.5 1.5 0 0 0 16 13V6a1.5 1.5 0 0 0-1.5-1.5h-13A1.5 1.5 0 0 0 0 6v7zM2 3a.5.5 0 0 0 .5.5h11a.5.5 0 0 0 0-1h-11A.5.5 0 0 0 2 3zm2-2a.5.5 0 0 0 .5.5h7a.5.5 0 0 0 0-1h-7A.5.5 0 0 0 4 1z"></path></svg>
                        {"Collections"}
                    </div>

                    <div class="sidebaritem pointer">
                        <svg stroke="currentColor" fill="currentColor" stroke-width="0" viewBox="0 0 24 24" height="25px" width="25px" xmlns="http://www.w3.org/2000/svg"><path fill="none" stroke-width="2" d="M8.9997,0.99999995 L8.9997,8.0003 L1.9997,20.0003 L1.9997,23.0003 L21.9997,23.0003 L21.9997,20.0003 L14.9997,8.0003 L14.9997,0.99999995 M15,18 C15.5522847,18 16,17.5522847 16,17 C16,16.4477153 15.5522847,16 15,16 C14.4477153,16 14,16.4477153 14,17 C14,17.5522847 14.4477153,18 15,18 Z M9,20 C9.55228475,20 10,19.5522847 10,19 C10,18.4477153 9.55228475,18 9,18 C8.44771525,18 8,18.4477153 8,19 C8,19.5522847 8.44771525,20 9,20 Z M18,13 C11,9.99999996 12,17.0000002 6,14 M5.9997,1.0003 L17.9997,1.0003"></path></svg>
                        {"Test"}
                    </div>
                </div>

                <div class="sidebar2">
                    <div>
                        <div class="pointer">
                            <svg viewBox="0 0 1024 1024" fill="currentColor" height="20px" width="20px" ><defs><style /></defs><path d="M482 152h60q8 0 8 8v704q0 8-8 8h-60q-8 0-8-8V160q0-8 8-8z" /><path d="M176 474h672q8 0 8 8v60q0 8-8 8H176q-8 0-8-8v-60q0-8 8-8z" /></svg>
                        </div>
                    </div>

                    <div class="sidebar2item sidebar2item-selected pointer">{"API 1"}</div>
                    <div class="sidebar2item pointer">{"API 2"}</div>
                    <div class="sidebar2item pointer">{"API 3"}</div>
                </div>

                <div class="content">
                    <div class="req">
                        <div class="requestbar">
                            <div class="">
                                <select id="methodselect" class="methodselect pointer">
                                    <option value="get">{"GET"}</option>
                                    <option value="post">{"POST"}</option>
                                </select>
                            </div>

                            <div>
                                <input id="urlinput" class="urlinput" type="text" placeholder="http://" />
                            </div>

                            <button class="sendbtn pointer" type="button" onclick={ctx.link().callback(|_| Msg::SendPressed)}>{"Send"}</button>
                        </div>

                        <div class="reqtabs">
                            <div ref={sel.req_body_tab_ref.clone()} class="tab pointer tabSelected" onclick={ctx.link().callback(|_| Msg::ReqBodyPressed)}>{"Body"}</div>
                            <div ref={sel.req_params_tab_ref.clone()} class="tab pointer" onclick={ctx.link().callback(|_| Msg::ReqParamsPressed)}>{"Params"}</div>
                            <div ref={sel.req_headers_tab_ref.clone()} class="tab pointer" onclick={ctx.link().callback(|_| Msg::ReqHeadersPressed)}>{"Headers"}</div>
                        </div>

                        <div class="tabcontent">
                            if sel.req_tab == 1 {
                                <textarea class="reqbody" placeholder="Request body">

                                </textarea>
                            } else if sel.req_tab == 2 {
                                <table>
                                    <tr>
                                        <th>{"Key"}</th>
                                        <th>{"Value"}</th>
                                    </tr>
                                    <tr>
                                        <td>{"Key"}</td>
                                        <td>{"Value"}</td>
                                    </tr>
                                    <tr>
                                        <td>{"Key"}</td>
                                        <td>{"Value"}</td>
                                    </tr>
                                </table>
                            } else if sel.req_tab == 3 {
                                <table>
                                    <tr>
                                        <th>{"Key"}</th>
                                        <th>{"Value"}</th>
                                    </tr>
                                    <tr>
                                        <td>{"Key"}</td>
                                        <td>{"Value"}</td>
                                    </tr>
                                    <tr>
                                        <td>{"Key"}</td>
                                        <td>{"Value"}</td>
                                    </tr>
                                </table>
                            }
                        </div>
                    </div>


                    <div class="resp">
                        <div class="respline">
                            <div class="resptabs">
                                <div ref={sel.resp_body_tab_ref.clone()} class="tab pointer tabSelected" onclick={ctx.link().callback(|_| Msg::RespBodyPressed)}>{"Body"}</div>
                                <div ref={sel.resp_headers_tab_ref.clone()} class="tab pointer" onclick={ctx.link().callback(|_| Msg::RespHeadersPressed)}>{"Headers"}</div>
                            </div>

                            <div class="respstats">
                                <div id="status" class="respstat">{"Status: 0"}</div>
                                <div id="time" class="respstat">{"Time: 0ms"}</div>
                                <div id="size" class="respstat">{"Size: 0B"}</div>
                            </div>
                        </div>

                        <div class="tabcontent">
                            if sel.resp_tab == 1 {
                                <textarea id="respbody" class="respbody" placeholder="Response body" readonly=true>

                                </textarea>
                            } else if sel.resp_tab == 2 {
                                <table>
                                    <tr>
                                        <th>{"Key"}</th>
                                        <th>{"Value"}</th>
                                    </tr>
                                    <tr>
                                        <td>{"Key"}</td>
                                        <td>{"Value"}</td>
                                    </tr>
                                    <tr>
                                        <td>{"Key"}</td>
                                        <td>{"Value"}</td>
                                    </tr>
                                </table>
                            }
                        </div>
                    </div>
                </div>
            </div>

            <div class="console">
                <p> {"Console"} </p>
                <p> {"this is a log"} </p>
            </div>

        </body>
        </>
    };
}
