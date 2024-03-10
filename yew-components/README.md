# Yew Component Architure

Yew support component architecture similar to React. The main difference is that Yew uses Rust's macro system to define components. This allows for a more Rust-like syntax and better type safety.

This give us a feel like we are using React.

## Components

Components are the building blocks of a Yew application. They are reusable and can be nested inside other components. Components are defined using the `html!` macro.


## How to run the code

- First make sure that you have installed Rust and Cargo. If not then install it from [here](https://www.rust-lang.org/tools/install).

- Then clone the repository and navigate to the directory.

- Make sure you install the recommended packages required for yew projects. [Check here for more information](https://yew.rs/docs/getting-started/introduction)

- Once you have installed the required packages, run the following command to start the server.

```bash
trunk serve
```