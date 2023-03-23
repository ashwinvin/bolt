## Table of Contents

- [Bolt](#bolt)
  - [Quick start 👩‍💻](#quick-start-)
  - [Walk through 🚶🏻](#walk-through-)
    - [Modules 🧩](#modules-)
  - [Our Contributors ✨](#our-contributors-)
  - [Support ⭐](#support-)

# Bolt

Native light ✨, optimized 🛠 and memory safe 🔐 http api client written in rust.

## Quick start 👩‍💻

> ⚠️Prerequisites
> 
> * Make sure you have [rust](https://www.rust-lang.org/tools/install) installed.
> 
> * Also make sure to follow [CONTRIBUTING guidelines](https://github.com/hiro-codes/bolt/blob/main/CONTRIBUTING.md)

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

<!-- ---

<a name="building-with-gitpod"></a>
## Building with Gitpod 💣

By using [Gitpod.io](https://www.gitpod.io), all the necessary dependencies will be installed
and the website will be built in one single click. No extra setup is required.

[![Gitpod Ready-to-Code](https://gitpod.io/button/open-in-gitpod.svg)](https://gitpod.io/#https://github.com/hiro-codes/bolt)

--- -->

## Our Contributors ✨

<a href="https://github.com/hiro-codes/bolt/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=hiro-codes/bolt" />
</a>

<!-- ## License 📝 -->

## Support ⭐
_We greatly appreciate your interest in our project! If you would like to contribute, we welcome any and all feedback, bug reports, and pull requests. Additionally, leaving a star on our repository lets us know that our work is valuable to you and helps others discover our project. Thank you for your support!⭐_
