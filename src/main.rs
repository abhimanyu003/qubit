#[macro_use]
extern crate lazy_static;
extern crate pest;
#[macro_use]
extern crate pest_derive;

use yew::ComponentLink;
use yew::prelude::*;
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
    total: f64,
}

fn transform(c: f64) -> String {
    use float_pretty_print::PrettyPrintFloat;
    if c.is_nan() {
        return "-".to_string();
    }
    if c.fract() == 0.0 {
        return c.to_string().trim().to_string();
    }
    return PrettyPrintFloat(c).to_string().trim().to_string();
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            text: String::from(""),
            total: 0.0,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::AddText(input) => {
                let mut output = "".to_string();
                let mut total = 0.0;
                let split = input.split('\n');

                for s in split {
                    let p = parser::parse(s);
                    if p.is_normal() {
                        total = total + p;
                    }
                    output = format!("{}{}", output, transform(p) + "\n");
                }

                self.total = total;
                self.text = output;
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
            <div class="min-h-screen px-3 bg-gray-100 md:px-0">
            <div class="flex flex-col items-center justify-center w-full h-full space-y-8">
                <div class="flex items-center justify-between w-full max-w-lg px-4 py-4 -mx-4 border-b border-gray-200 sm:mx-0 sm:px-0">
                    <h1 class="text-2xl font-extrabold tracking-tight text-gray-900">{ title }</h1>
                    <div class="flex items-center ml-6 space-x-6 sm:space-x-10 sm:ml-10">
                    <a href="https://github.com/abhimanyu003/qubit" target="_blank" class="text-gray-600 transition-colors duration-200 hover:text-gray-900">
                        { "Documentation" }
                    </a>
                    <a href="https://github.com/abhimanyu003/qubit" target="_blank" class="text-gray-600 transition-colors duration-200 hover:text-gray-900">
                        <span class="hidden sm:inline"></span>
                        <svg width="24" height="24" viewBox="0 0 16 16" fill="currentColor"><path fill-rule="evenodd" d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.013 8.013 0 0016 8c0-4.42-3.58-8-8-8z"></path></svg>
                    </a>
                    </div>
                </div>
        
                <div class="w-full max-w-lg subpixel-antialiased tracking-tight text-gray-500 bg-gray-900 rounded-md shadow-2xl text-md">
                    <div class="grid h-full grid-cols-4 p-4">
                        <div class="col-span-3">
                            <textarea oninput=self.link.callback(|e: InputData| Msg::AddText(e.value)) class="w-full h-full text-gray-200 placeholder-gray-500 placeholder-opacity-50 bg-transparent border-0 appearance-none resize-none focus:outline-none focus:ring-0 focus:border-0 active:border-0" style="resize: none" data-gramm="false" placeholder="2 + 2 + sin ( 90 )\n12 kg to g"></textarea>
                        </div>
                        <div class="col-span-1 overflow-x-auto text-right text-green-300 border-l border-opacity-10">
                            <div> {
                                for self.text.split('\n').into_iter().map(|v| {
                                    html!{
                                        <div class="w-full ">{ v }</div>
                                    } })
                                }
                            </div>
                            <div class="pt-5 text-sm text-gray-600">{ "Total: " }{ self.total }</div>
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
