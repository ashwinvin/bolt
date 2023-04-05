use crate::send_request;
use crate::utils::*;
use crate::BoltContext;
use crate::Collection;
use crate::Msg;
use crate::Page;
use crate::Request;

// FIXME: Collection checks

pub fn process(bctx: &mut BoltContext, msg: Msg) -> bool {
    match msg {
        Msg::SelectedMethod(meth) => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            let current = bctx.main_current;
            bctx.main_col.requests[current].method = meth;

            return true;
        }

        Msg::SendPressed => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            let current = bctx.main_current;
            bctx.main_col.requests[current].request_index = current;

            // if state.requests[current].body != "" {
            send_request(bctx.main_col.requests[bctx.main_current].clone());
            // }

            return true;
        }

        Msg::HelpPressed => {
            open_link("https://github.com/hiro-codes/bolt".to_string());

            return true;
        }

        Msg::ReqBodyPressed => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            bctx.req_tab = 1;

            switch_req_tab(1);
            return true;
        }

        Msg::ReqHeadersPressed => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            bctx.req_tab = 3;

            switch_req_tab(3);
            return true;
        }

        Msg::ReqParamsPressed => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            bctx.req_tab = 2;

            switch_req_tab(2);
            return true;
        }

        Msg::RespBodyPressed => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            bctx.resp_tab = 1;

            switch_resp_tab(1);
            return true;
        }

        Msg::RespHeadersPressed => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            bctx.resp_tab = 2;

            switch_resp_tab(2);
            return true;
        }

        Msg::ReceivedResponse => {
            return true;
        }

        Msg::AddHeader => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            let current = bctx.main_current;
            bctx.main_col.requests[current]
                .headers
                .push(vec!["".to_string(), "".to_string()]);

            return true;
        }

        Msg::RemoveHeader(index) => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            let current = bctx.main_current;
            bctx.main_col.requests[current].headers.remove(index);

            return true;
        }

        Msg::AddParam => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            let current = bctx.main_current;
            bctx.main_col.requests[current]
                .params
                .push(vec!["".to_string(), "".to_string()]);

            return true;
        }

        Msg::AddCollection => {
            // let mut state = GLOBAL_STATE.lock().unwrap();

            let mut new_collection = Collection::new();

            new_collection.name = new_collection.name + &(bctx.collections.len() + 1).to_string();
            new_collection.requests.push(Request::new());

            bctx.collections.push(new_collection);

            return true;
        }

        Msg::RemoveCollection(index) => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            bctx.collections.remove(index);

            bctx.col_current = vec![0, 0];

            return true;
        }

        Msg::RemoveParam(index) => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            let current = bctx.main_current;
            bctx.main_col.requests[current].params.remove(index);

            return true;
        }

        Msg::MethodChanged => {
            let method = get_method();

            // let mut state = GLOBAL_STATE.lock().unwrap();
            let current = bctx.main_current;
            bctx.main_col.requests[current].method = method;

            return true;
        }

        Msg::UrlChanged => {
            let url = get_url();

            // let mut state = GLOBAL_STATE.lock().unwrap();

            if bctx.page == Page::Home {
                let current = bctx.main_current;
                bctx.main_col.requests[current].url = url.clone();
                bctx.main_col.requests[current].name = url;
            } else {
                let current = &bctx.col_current;
                bctx.collections[current[0]].requests[current[1]].url = url.clone();
                bctx.collections[current[0]].requests[current[1]].name = url;
            }

            return true;
        }

        Msg::BodyChanged => {
            let body = get_body();

            // let mut state = GLOBAL_STATE.lock().unwrap();
            let current = bctx.main_current;
            bctx.main_col.requests[current].body = body;

            return true;
        }

        Msg::HeaderChanged(index) => {
            let header = get_header(index);

            // let mut state = GLOBAL_STATE.lock().unwrap();
            let current = bctx.main_current;
            bctx.main_col.requests[current].headers[index] = header;

            return true;
        }

        Msg::ParamChanged(index) => {
            let param = get_param(index);

            // let mut state = GLOBAL_STATE.lock().unwrap();
            let current = bctx.main_current;
            bctx.main_col.requests[current].params[index] = param;

            return true;
        }

        Msg::AddRequest => {
            // let mut state = GLOBAL_STATE.lock().unwrap();

            let mut new_request = Request::new();
            new_request.name = new_request.name + &(bctx.main_col.requests.len() + 1).to_string();

            bctx.main_col.requests.push(new_request);

            return true;
        }

        Msg::AddToCollection(index) => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            let collection = &mut bctx.collections[index];

            let mut new_request = Request::new();
            new_request.name = new_request.name + &(collection.requests.len() + 1).to_string();

            collection.requests.push(new_request);

            return true;
        }

        Msg::RemoveRequest(index) => {
            // let mut state = GLOBAL_STATE.lock().unwrap();

            if bctx.main_col.requests.len() > 1 {
                bctx.main_col.requests.remove(index);
            } else {
                bctx.main_col.requests = vec![Request::new()];
            }

            if bctx.main_current > bctx.main_col.requests.len() - 1 {
                bctx.main_current = bctx.main_col.requests.len() - 1;
            }

            return true;
        }

        Msg::RemoveFromCollection(col_index, req_index) => {
            // let mut state = GLOBAL_STATE.lock().unwrap();

            if bctx.collections[col_index].requests.len() > 1 {
                bctx.collections[col_index].requests.remove(req_index);
            } else {
                bctx.collections[col_index].requests = vec![Request::new()];
            }

            bctx.col_current = vec![0, 0];

            return true;
        }

        Msg::SelectRequest(index) => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            bctx.main_current = index;

            return true;
        }

        Msg::SelectFromCollection(col_index, req_index) => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            bctx.col_current = vec![col_index, req_index];

            return true;
        }

        Msg::Update => {
            return true;
        }

        Msg::SwitchPage(page) => {
            // let mut state = GLOBAL_STATE.lock().unwrap();
            bctx.page = page;

            return true;
        }
    }
}
