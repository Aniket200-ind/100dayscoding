# Yew counter app

This is a simple counter app built with Yew, a modern Rust framework for building client web apps. It is a simple app that increments and decrements a counter. The counter is stored in the app's state and is updated using Yew's message passing system. The counter is displayed in the app's view.

**Note:** I have taken an extra step and decided to implement my favorite CSS framework, TailwindCSS, to style the app. This is not a requirement for the challenge, but I wanted to demonstrate that Yew can be used with TailwindCSS.

---

## How to run this app?

- First you need to install Rust and Cargo. You can install it from [here](https://www.rust-lang.org/tools/install).

 - Then visit yew's official website and follow the instructions to install wasm-pack and cargo-generate. You can find the instructions [here](https://yew.rs/docs/getting-started/introduction).

- After installing all the required packages, you can run this app by running the following command in your terminal:

```bash
trunk serve
```