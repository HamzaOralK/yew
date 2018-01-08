extern crate chrono;
#[macro_use]
extern crate yew;

use chrono::prelude::*;
use yew::html::*;
use yew::component::Component;
use yew::services::console::ConsoleService;

struct Context {
    console: ConsoleService,
}

struct Model {
    value: i64,
}

enum Msg {
    Increment,
    Decrement,
    Bulk(Vec<Msg>),
}

impl Component<Context> for Model {
    type Msg = Msg;

    fn update(&mut self, msg: Msg, context: &mut LocalSender<Context, Msg>) {
        match msg {
            Msg::Increment => {
                self.value = self.value + 1;
                context.console.log("plus one");
            }
            Msg::Decrement => {
                self.value = self.value - 1;
                context.console.log("minus one");
            }
            Msg::Bulk(list) => {
                for msg in list {
                    self.update(msg, context);
                }
            }
        }
    }

    fn view(&self) -> Html<Context, Msg> {
        html! {
            <div>
                <nav class="menu",>
                    <button onclick=|_| Msg::Increment,>{ "Increment" }</button>
                    <button onclick=|_| Msg::Decrement,>{ "Decrement" }</button>
                    <button onclick=|_| Msg::Bulk(vec!(Msg::Increment, Msg::Increment)),>{ "Increment Twice" }</button>
                </nav>
                <p>{ self.value }</p>
                <p>{ Local::now() }</p>
            </div>
        }
    }
}

impl Default for Model {
    fn default() -> Self {
        Model {
            value: 0,
        }
    }
}

fn main() {
    use std::rc::Rc;
    use std::cell::RefCell;
    yew::initialize();
    let context = Context {
        console: ConsoleService,
    };
    let mut app = App::new(Rc::new(RefCell::new(context)));
    app.mount(Model::default());
    yew::run_loop();
}
