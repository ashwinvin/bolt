## Table of Contents

- [Bolt](#bolt)
  - [Tech stack 📚](#tech-stack-)
  - [Socials 📱](#socials-)
  - [Quick start 👩‍💻](#quick-start-)
  - [Walk through 🚶🏻](#walk-through-)
    - [Modules 🧩](#modules-)
  - [Road Map 🚧](#road-map-)
  - [Building with Gitpod 💣](#building-with-gitpod-)
  - [Our Contributors ✨](#our-contributors-)
  - [Support ⭐](#support-)

# Bolt

<!-- Native light ✨, optimized 🛠 and memory safe 🔐 http api client written in rust. -->

Bolt is the ultimate HTTP API client, memory safe 🔐, crafted with Rust's 🦀  native power 💪 and optimized 🛠 for lightning-fast performance ✨, all while ensuring your data stays secure. Experience the seamless simplicity and safety of Bolt 🛡.

## Tech stack 📚

<p>
<!-- rust stack -->
  <a href="http://rust-lang.org">
    <img src="https://img.shields.io/badge/rust-7c3aed?style=for-the-badge&logo=rust&logoColor=white" alt="Rust Official Website"/>
  </a>
  <!-- end of rust stack -->

  <!-- yew stack -->
  <a href="https://yew.rs">
    <img src="https://img.shields.io/badge/yew-7c3aed?style=for-the-badge&logo=yew&logoColor=white" alt="Yew"/>
  </a>
  <!-- end of Yew stack -->

  <!-- Tauri stack -->
  <a href="https://tauri.app">
    <img src="https://img.shields.io/badge/Tauri-7c3aed?style=for-the-badge&logo=tauri&logoColor=white" alt="Tauri"/>
  </a>
  <!-- end of tauri stack -->

  <!-- html 5 stack -->
   <a href="https://developer.mozilla.org/en-US/docs/Web/HTML">
    <img src="https://img.shields.io/badge/html-7c3aed?style=for-the-badge&logo=html5&logoColor=white" alt="HTML"/>
  <!-- End of html stack -->

  <!-- css 3 stack -->
   <a href="https://developer.mozilla.org/en-US/docs/Web/HTML">
    <img src="https://img.shields.io/badge/CSS-7c3aed?style=for-the-badge&logo=css3&logoColor=white" alt="CSS 3"/>
  <!-- End of css 3 stack -->

  <!-- Javascript stack -->
   <a href="https://developer.mozilla.org/en-US/docs/Web/JavaScript">
    <img src="https://img.shields.io/badge/javascript-7c3aed?style=for-the-badge&logo=javascript&logoColor=white" alt="Javascript"/>
  </a>
  <!-- end of javascript -->
</p>

## Socials 📱

<a href="http://twitter.com/hiro_codes" target="blank">
<img src="https://cdn-icons-png.flaticon.com/32/733/733579.png" alt="twitter">
</a>
<a href="https://discord.gg/bujCS6srrV" target="blank">
<img src="https://cdn-icons-png.flaticon.com/32/5968/5968756.png" alt="discord">
</a>

## Quick start 👩‍💻

> ⚠️Prerequisites
> 
> * Make sure you have [rust](https://www.rust-lang.org/tools/install) installed.
> 
> * Also make sure to follow [CONTRIBUTING guidelines](https://github.com/hiro-codes/bolt/blob/master/CONTRIBUTING.md)

**Clone Repository**
> ``` bash
> # clone with http
> git clone https://github.com/hiro-codes/bolt
> # or with SSH
> git clone git@github.com:<username>/hiro-bolt.git
> ```

## Walk through 🚶🏻

> ``` bash
> cd bolt # navigate to the project.
> ```

> ``` bash
> make setup # install requizred dependencies.
> ```

> ``` bash
> make run # Build and run modules in development in watch mode.
> ```

> ``` bash
> make web # start web server.
> ```

> ``` bash
> make build # generate production binaries.
> ```


### Modules 🧩

> Tauri is a backend framework that handles the integration of your Yew-based front-end with the desktop environment. It provides functionality such as networking and communication with the operating system. It packages your Yew front-end as a desktop application that can be installed and run on the user's computer.
>
> Together, the Yew and Tauri modules allow you to build a powerful, full-featured desktop application using Rust as the primary language.
* #### ***Tauri***:
    This module is responsible for implementing the business logic and functionality of bolt. It contains the core code that runs the application's features and processes user inputs.

* #### ***Yew***:
    This module is responsible for building the bolt user interface using Rust, HTML, CSS, and JavaScript. It allows you to create a responsive and dynamic front-end that can interact with the user and display information. Once the front-end is built, it is given to Tauri.

---

## Road Map 🚧
 * [x] Http Request
 * [ ] Save Request Example
 * [ ] Web
 * [ ] Tabs Support
 * [ ] Collection
 * [ ] Generate Docs
 * [ ] todos.iter().chain(&others).map(|&others|others).collect();; 
 <!-- * [ ] [&others[..]].concat(); -->

<a name="building-with-gitpod"></a>
## Building with Gitpod 💣

By using [Gitpod.io](https://www.gitpod.io), all the necessary dependencies will be installed
and the website will be built in one single click. No extra setup is required.

[![Gitpod Ready-to-Code](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/hiro-codes/bolt)

---

## Our Contributors ✨

<a href="https://github.com/hiro-codes/bolt/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=hiro-codes/bolt" />
</a>

<!-- ## License 📝 -->

## Support ⭐

_We greatly appreciate your interest in our project! If you would like to contribute, we welcome any and all feedback, bug reports, and pull requests. Additionally, leaving a star on our repository lets us know that our work is valuable to you and helps others discover our project. Thank you for your support!⭐_

<a href="https://www.buymeacoffee.com/0xhiro" target="_blank"><img src="https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png" alt="Buy Me A Coffee" style="height: 41px !important;width: 174px !important;box-shadow: 0px 3px 2px 0px rgba(190, 190, 190, 0.5) !important;-webkit-box-shadow: 0px 3px 2px 0px rgba(190, 190, 190, 0.5) !important;" ></a>