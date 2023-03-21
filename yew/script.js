// access the pre-bundled global API functions
const { invoke } = window.__TAURI__.tauri
const { listen } = window.__TAURI__.event

export function bolt_log(log) {
  invoke('bolt_log', { log: log })
};

export function send_request(url, method) {
  invoke('send_request', { url: url, method: method })
}

export function get_method() {
  const selectElement = document.getElementById("methodselect");
  const selectedValue = selectElement.value;

  return selectedValue
}

export function get_url() {
  const selectElement = document.getElementById("urlinput");
  const selectedValue = selectElement.value;

  return selectedValue
}

export function set_respbody(content) {
  const body = document.getElementById("respbody");
  body.value = content;
}


await listen('receive_response', (event) => {
  bolt_log("received a response");

  set_respbody(event.payload.body);
  // yew_func()
})
