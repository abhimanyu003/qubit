#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use yew::prelude::*;
use yew::services::ConsoleService;
use yew::ComponentLink;
use yew::ShouldRender;

mod convert_chart;
mod parser;
mod test;

enum Msg {
    AddText(String),
}

struct Model {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    text: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            text: String::from(""),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddText(val) => {
                let mut input = "".to_string();
                let split = val.split('\n');
                for s in split {
                    input = format!("{}{}", input, parser::transform(s.to_string()) + "\n");
                }
                self.text = input;
                ConsoleService::info(format!("Update: {:?}", self.text).as_ref());
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        false
    }

    fn view(&self) -> Html {
        let title = "Qubit".to_string();
        return html! {
            <div>
            <div class="w-full h-full flex flex-col items-center justify-center space-y-8 pt-10">
                <h1 class="text-2xl sm:text-2xl lg:text-4xl leading-none font-extrabold text-gray-900 tracking-tight mb-8">
                        { title }
                    </h1>
                <div class="w-full max-w-lg bg-gray-900 text-gray-500 rounded-md text-sm">
                    <div class="h-80 p-4 grid grid-cols-4">
                        <div class="col-span-3">
                            <textarea oninput=self.link.callback(|e: InputData| Msg::AddText(e.value)) class="text-white w-full h-full bg-transparent focus:outline-none appearance-none border-0 focus:ring-0 focus:border-0 active:border-0 text-sm" style="resize: none" data-gramm="false"></textarea>
                        </div>
                        <div class="col-span-1 px-4 py-2 border-l border-opacity-10 text-green-300 overflow-x-auto text-right">
                            <div> {
                                for self.text.split('\n').into_iter().map(|v| {
                                    html!{
                                        <div class="w-full ">{ v }</div>
                                    } })
                                }
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        </div>
        };
    }
}

fn main() {
    yew::start_app::<Model>();
}
