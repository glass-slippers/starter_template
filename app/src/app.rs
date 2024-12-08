use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <MetaTags/>
                <Stylesheet id="leptos" href="/pkg/starter-template.css"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {

        // sets the document title
        <Title text="Welcome to Leptos"/>



        // content for this welcome page
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("") view=HomePage/>
                </Routes>
            </main>
        </Router>
    }
}

#[server(endpoint = "fff")]
pub async fn fff(i: i32) -> Result<i32, ServerFnError> {
    Ok(i + 1)
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    // Creates a reactive value to update the button
    let count = RwSignal::new(0);
    let a = ServerAction::<Fff>::new();
    Effect::new(move |_| {
        a.value().get().map(|a| a.map(|a| *count.write() = a));
    });
    let on_click = move |_| {
        a.dispatch(Fff { i: *count.read() });
    };

    view! {
        <h1 class="text-xl">"Welcome to Leptos!"</h1>
        <button on:click=on_click>"Click Me: " {move || count.get()}</button>
    }
}
