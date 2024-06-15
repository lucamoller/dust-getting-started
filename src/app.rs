// use crate::error_template::{AppError, ErrorTemplate};
use dust::leptos::logging::log;
use dust::leptos::*;
use dust::leptos_meta::*;
use dust::{dust_define_client_callback, dust_define_server_callback, DustState, Input, Output};

#[derive(Clone, DustState)]
#[dust_register_callback(update_sum)]
#[dust_register_callback(print_sum)]
#[dust_register_callback(update_sum_of_squares)]
#[dust_register_callback(update_squared_sum)]
#[dust_register_callback(update_text_summary)]
pub struct MyState {
    pub value1: i32,
    pub value2: i32,
    pub sum: i32,
    pub sum_of_squares: i32,
    pub squared_sum: i32,
    pub text_summary: String,
}

impl Default for MyState {
    fn default() -> MyState {
        MyState {
            value1: 1,
            value2: 1,
            sum: 0,
            sum_of_squares: 0,
            squared_sum: 0,
            text_summary: String::from(""),
        }
    }
}

#[dust_define_server_callback(MyState)]
fn update_sum(value1: Input<i32>, value2: Input<i32>, sum: &mut Output<i32>) {
    sum.set(value1.value + value2.value);
}

#[dust_define_server_callback(MyState)]
fn print_sum(sum: Input<i32>) {
    println!("new sum {}", sum.value);
}

#[dust_define_server_callback(MyState)]
fn update_sum_of_squares(value1: Input<i32>, value2: Input<i32>, sum_of_squares: &mut Output<i32>) {
    sum_of_squares.set(value1.value * value1.value + value2.value * value2.value);
}

#[dust_define_server_callback(MyState)]
fn update_squared_sum(sum: Input<i32>, squared_sum: &mut Output<i32>) {
    squared_sum.set(sum.value * sum.value);
}

#[dust_define_client_callback(MyState)]
fn update_text_summary(
    value1: Input<i32>,
    value2: Input<i32>,
    sum: Input<i32>,
    sum_of_squares: Input<i32>,
    squared_sum: Input<i32>,
    text_summary: &mut Output<String>,
) {
    text_summary.set(format!(
        "value1: {}, value2: {}, sum: {}, sum_of_squares: {}, squared_sum: {}",
        value1.value, value2.value, sum.value, sum_of_squares.value, squared_sum.value
    ));
}

#[component]
pub fn App() -> impl IntoView {
    log!("Rendering App...");
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    MyState::provide_and_initialize_context();

    let state = MyState::expect_context();

    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/dust-leptos-single.css"/>

        // sets the document title
        <Title text="Welcome to Dust"/>

        <h1>"Welcome to Dust!"</h1>
        <button on:click=state.increment_onclick_value1()>"Increment value1: " {state.value1}</button>
        <button on:click=state.increment_onclick_value2()>"Increment value2: " {state.value2}</button>
        <p> Summary: {state.text_summary} </p>
    }
}
