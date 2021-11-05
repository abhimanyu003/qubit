#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use yew::ComponentLink;
use yew::prelude::*;
use yew::services::ConsoleService;
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
        let title = "qubit".to_string();
        return html! {
        <div class="min-h-screen bg-gray-100">

            <div class="w-full h-full flex flex-col items-center justify-center space-y-8 pb-10">
                <div class="border-b border-gray-200 py-4 flex items-center justify-between -mx-4 px-4 sm:mx-0 sm:px-0 w-full max-w-2xl">
                    <h1 class="text-2xl font-extrabold tracking-tight text-gray-900">{ title }</h1>
                    <div class="flex items-center space-x-6 sm:space-x-10 ml-6 sm:ml-10">
                    <a href="https://github.com/abhimanyu003/qubit" target="_blank" class="text-gray-600 hover:text-gray-900 transition-colors duration-200">
                        { "Documentation" }
                    </a>
                    <a href="https://github.com/abhimanyu003/qubit" target="_blank" class="text-gray-600 hover:text-gray-900 transition-colors duration-200">
                        <span class="hidden sm:inline"></span>
                        <svg width="24" height="24" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path></svg>
                    </a>
                    </div>
                </div>

                <div class="w-full max-w-2xl bg-gray-900 text-gray-500 rounded-md text-md shadow-2xl tracking-tight subpixel-antialiased">
                    <div class="h-full p-4 grid grid-cols-4">
                        <div class="col-span-3">
                            <textarea oninput=self.link.callback(|e: InputData| Msg::AddText(e.value)) class="text-gray-200 w-full h-full bg-transparent focus:outline-none appearance-none border-0 focus:ring-0 focus:border-0 active:border-0 placeholder-opacity-50 placeholder-gray-500 resize-none" style="resize: none" data-gramm="false" placeholder="2 + 2 + sin ( 90 )\n12 kg to g"></textarea>
                        </div>
                        <div class="col-span-1 border-l border-opacity-10 text-green-300 overflow-x-auto text-right">
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
