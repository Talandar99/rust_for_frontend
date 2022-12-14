use yew::prelude::*;

enum Msg {
    AddOne,
    SubstractOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::SubstractOne => {
                self.value -= 1;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
           <body>
                <h2>{"Rust is Inevitable"}</h2>
                <h1>{ "🦀" }</h1>
                <h2>{"Join Crab Cult"}</h2>
                <div>
                    <p><button onclick={link.callback(|_| Msg::SubstractOne)}>{ " -🦀 " }</button></p>
                    <p>{"__{"}</p><p>{ self.value }</p><p>{"}__"}</p>
                    <p><button onclick={link.callback(|_| Msg::AddOne)}>{ " +🦀 " }</button></p>
                </div>
            </body>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
