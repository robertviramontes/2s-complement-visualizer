use web_sys::HtmlInputElement;
use yew::prelude::*;
use regex::Regex;

#[derive(Properties, PartialEq)]
struct NumberDisplayProps {
    digits: AttrValue,
}

#[function_component(NumberDisplay)]
fn number_display(NumberDisplayProps { digits }: &NumberDisplayProps) -> Html {
    let char_vec = digits.chars();

    let digit_cells = char_vec.clone().map(|digit| html! {
        <th>{digit}</th>
    }).collect::<Html>();
    let base: i32 = 2;

    let value_cells = (1..=digits.len()).rev().map(|position| html! {
        <td> {
            if position == digits.len()
                {format!("{}", -1*(base.pow((position as u32) - 1)))
            } else {
                {format!("+{}", base.pow((position as u32) - 1))}
                } 
        }</td>
    }).collect::<Html>();

    let decimal_value = (1..=digits.len()).rev().zip(char_vec.clone()).map(|(position, digit)| {
        let position_value = if position == digits.len() {-1*(base.pow((position as u32) - 1))} else {base.pow((position as u32) - 1)};
        if digit == '1' {position_value} else {0}
    }).reduce(|a, b| a+b);

    html! {
    <>
        {match decimal_value {
            Some(value) => html! {<h2> {format!("You entered the decimal number: {}", value)} </h2>},
            None => html!{<></>}
        }}
        
        <table>
            <tr>
                {digit_cells}
            </tr>
            <tr>
                {value_cells}
            </tr>
        </table>
    </>
    }
}

#[function_component(App)]
fn app() -> Html {
    let user_digits = use_state(|| String::new());
    let digits = use_state(|| String::new());
    let err_msg = use_state(|| String::new());

    let on_user_digit_input = {
        let user_digits = user_digits.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into::<HtmlInputElement>();

            user_digits.set(input.value());
        })
    };

    let on_submit = {
        let digits = digits.clone();
        let user_digits = user_digits.clone();
        let err_msg = err_msg.clone();
        let form_validation = Regex::new(r"[^01]+").unwrap();
        
        Callback::from(move |e: MouseEvent| {
            let is_input_invalid = form_validation.is_match(user_digits.as_str());
            if is_input_invalid {
                digits.set(String::new());
                user_digits.set(String::new());
                err_msg.set("Only enter a binary number.".to_string());
            } else {
                digits.set(user_digits.to_string());
                err_msg.set(String::new())
            }
        })
    };

    html! {
    <>
        <h1>{ "Two's Complement Visualizer" }</h1>
        <label for="binary_input">{ "Enter your binary number!" }</label>
        <input
            type="text"
            id="binary_input"
            title="only inputs 0s and 1s (binary number)"
            oninput={on_user_digit_input}
            name="binary_input"
            required=true
            minlength="1"
            maxlength="8"
            pattern="[0-1]*"
            size="10" />
        <input type="button" value="Submit" onclick={on_submit}/>
        <p>{ err_msg.to_string() } </p>
        <NumberDisplay digits={digits.to_string()} />
    </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
