// access the pre-bundled global API functions
const { invoke} = window.__TAURI__.tauri
const { listen, event } = window.__TAURI__.event

export function bolt_log(log) {
      invoke('bolt_log', {log: log})
}

export function send_btn() {
      invoke('send_btn', {})
}

const unlisten = await listen('yew_func', (event) => {
      bolt_log("reached JS");
 
      run();
})



function run() {

}