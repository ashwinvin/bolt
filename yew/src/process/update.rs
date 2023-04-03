use crate::send_request;
use crate::utils::*;
use crate::Msg;
use crate::Request;
use crate::GLOBAL_STATE;

pub fn process(msg: Msg) -> bool {
    match msg {
        Msg::SelectedMethod(meth) => {
            let mut state = GLOBAL_STATE.lock().unwrap();
            let current = state.current_request;
            state.requests[current].method = meth;

            return true;
        }

        Msg::SendPressed => {
            let state = GLOBAL_STATE.lock().unwrap();
            send_request(state.requests[state.current_request].clone());

            return true;
        }

        Msg::ReqBodyPressed => {
            let mut state = GLOBAL_STATE.lock().unwrap();
            state.req_tab = 1;

            switch_req_tab(1);
            return true;
        }

        Msg::ReqHeadersPressed => {
            let mut state = GLOBAL_STATE.lock().unwrap();
            state.req_tab = 3;

            switch_req_tab(3);
            return true;
        }

        Msg::ReqParamsPressed => {
            let mut state = GLOBAL_STATE.lock().unwrap();
            state.req_tab = 2;

            switch_req_tab(2);
            return true;
        }

        Msg::RespBodyPressed => {
            let mut state = GLOBAL_STATE.lock().unwrap();
            state.resp_tab = 1;

            switch_resp_tab(1);
            return true;
        }

        Msg::RespHeadersPressed => {
            let mut state = GLOBAL_STATE.lock().unwrap();
            state.resp_tab = 2;

            switch_resp_tab(2);
            return true;
        }

        Msg::ReceivedResponse => {
            return true;
        }

        Msg::AddHeader => {
            let mut state = GLOBAL_STATE.lock().unwrap();
            let current = state.current_request;
            state.requests[current]
                .headers
                .push(vec!["".to_string(), "".to_string()]);

            return true;
        }

        Msg::RemoveHeader(index) => {
            let mut state = GLOBAL_STATE.lock().unwrap();
            let current = state.current_request;
            state.requests[current].headers.remove(index);

            return true;
        }

        Msg::AddParam => {
            let mut state = GLOBAL_STATE.lock().unwrap();
            let current = state.current_request;
            state.requests[current]
                .params
                .push(vec!["".to_string(), "".to_string()]);

            return true;
        }

        Msg::RemoveParam(index) => {
            let mut state = GLOBAL_STATE.lock().unwrap();
            let current = state.current_request;
            state.requests[current].params.remove(index);

            return true;
        }

        Msg::MethodChanged => {
            let method = get_method();

            let mut state = GLOBAL_STATE.lock().unwrap();
            let current = state.current_request;
            state.requests[current].method = method;

            return true;
        }

        Msg::UrlChanged => {
            let url = get_url();

            let mut state = GLOBAL_STATE.lock().unwrap();
            let current = state.current_request;
            state.requests[current].url = url.clone();
             state.requests[current].name = url;

            return true;
        }

        Msg::BodyChanged => {
            let body = get_body();

            let mut state = GLOBAL_STATE.lock().unwrap();
            let current = state.current_request;
            state.requests[current].body = body;

            return true;
        }

        Msg::HeaderChanged(index) => {
            let header = get_header(index);

            let mut state = GLOBAL_STATE.lock().unwrap();
            let current = state.current_request;
            state.requests[current].headers[index] = header;

            return true;
        }

        Msg::ParamChanged(index) => {
            let param = get_param(index);

            let mut state = GLOBAL_STATE.lock().unwrap();
            let current = state.current_request;
            state.requests[current].params[index] = param;

            return true;
        }

        Msg::AddRequest => {
            let mut state = GLOBAL_STATE.lock().unwrap();

            let mut new_request = Request::new();
            new_request.name = new_request.name + &(state.requests.len() + 1).to_string();

            state.requests.push(new_request);

            return true;
        }

        Msg::RemoveRequest(index) => {
            let mut state = GLOBAL_STATE.lock().unwrap();

            if state.requests.len() > 1 {
                state.requests.remove(index);
            }

            state.current_request = 0;

            return true;
        }

        Msg::SelectRequest(index) => {
            let mut state = GLOBAL_STATE.lock().unwrap();
            state.current_request = index;

            return true;
        }

        Msg::Update => {
            return true;
        }
    }
}
