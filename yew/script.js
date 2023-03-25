// access the pre-bundled global API functions
// const { invoke } = window.__TAURI__.tauri
// const { listen } = window.__TAURI__.event

// export function bolt_log(log) {
//   invoke('bolt_log', { log: log })
// };

// export function set_status(status) {
//   const div = document.getElementById("status");
//   div.innerHTML = "Status: " + status;
// }

// export function set_time(time) {
//   const div = document.getElementById("time");
//   div.innerHTML = "Time: " + time + " ms";
// }

// export function set_size(size) {
//   const div = document.getElementById("size");
//   div.innerHTML = "Size: " + size + " B";
// }

// await listen('receive_response', (event) => {
//   bolt_log("received a response");

//   set_respbody(event.payload.body);
//   set_status(event.payload.status);
//   set_time(event.payload.time);
//   set_size(event.payload.size);
// })
