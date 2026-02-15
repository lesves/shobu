use web_sys::HtmlInputElement;
use yew::prelude::*;
use std::fmt;

pub trait SelectItem = Clone + PartialEq + fmt::Display;

#[derive(Properties, PartialEq, Clone)]
pub struct SelectProps<T> where T: SelectItem {
	pub name: AttrValue,
    pub options: Vec<T>,

    #[prop_or_default]
    pub on_select: Option<Callback<T>>,
    #[prop_or_default]
    pub default: Option<T>,
}

#[function_component]
pub fn Select<T>(props: &SelectProps<T>) -> Html where T: SelectItem + 'static {
	let content = props.options.iter().enumerate().map(|(i, opt)| html!(
		<option value={i.to_string()} selected={if Some(opt) == props.default.as_ref() {true} else {false}}>{format!("{}", opt)}</option>
	)).collect::<Html>();

	let props_: SelectProps<T> = props.clone();
	let on_change = Callback::from(move |e: Event| {
		let val = e.target_unchecked_into::<HtmlInputElement>().value();
		let selected = props_.options[val.parse::<usize>().unwrap()].clone();
		if let Some(callback) = props_.on_select.clone() {
			callback.emit(selected);
		}
	});

    html! {
		<select name={props.name.clone()} onchange={on_change}>
			{content}
		</select>
    }
}
