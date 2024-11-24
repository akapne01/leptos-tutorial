use leptos::*;

/// Shows progress towards a goal.
#[component]
fn ProgressBar(
    /// The maximum value of the progress bar.
    #[prop(default = 100)] max: u16, // Optional, if not specified, default value is used
    /// How much progress should be displayed.
    #[prop(into)] progress: Signal<i32> // Automatically calls .into() on the values passed
    // Signal is an enumerated type: any kind of readable reactive signal.
    // MaybeSignal allows to use either static or reactive value.
) -> impl IntoView {
    view! { <progress max=max value=progress /> }
}

/*
    #[component] annotates a function that can be used as a component in Leptos application.
    Every component is a function that takes zero or more arguments of any type and returns
    impl IntoView.
    Component fn arguments are gathered together into a single props struct which is built
    by the view macro as needed.

    Body of the component runs once. Typically used to create few reactive variables and 
    define any side effects that run in response to those values changing, and describe UI. 
*/

#[component]
fn App() -> impl IntoView {
    /*  
        Signal is a basic unit of reactive change and state management.
        Returns the (getter, setter) tuple.
        Access current value: count.get() (clones the value)
        To set a current value: set_count.set(3) (overrides the value)
        In many cases, it is more efficient to use .with() or .update()
    */
    let (count, set_count) = create_signal(0);
    let (x, set_x) = create_signal(0);
    let double_count = move || count.get() * 2;
    let html = "<p>This HTML will be injected.</p>";
    /*
        defines user interfaces using a JSX-like format via the view macro.
     */
    view! {
        <div>
            <button
                on:click=move |_| {
                    set_count.update(|n| *n += 1);
                }
                // the class: syntax reactively updates a single class
                // here, we'll set the `red` class when `count` is odd
                class:red=move || count.get() % 2 == 1
            >
                "Click Me: "
                {move || count.get()}
            </button>

            <p>"Count: "{count}</p>
            <ProgressBar max=50 progress=count />

            <p>"Double Count: "{double_count}</p>
            <ProgressBar max=50 progress=Signal::derive(double_count) />
        </div>

        <div inner_html=html />

        <div>
            <button
                on:click=move |_| {
                    set_x.update(|n| *n += 10);
                }
                // set the `style` attribute
                style="position: absolute"
                // and toggle individual CSS properties with `style:`
                style:left=move || format!("{}px", x.get() + 100)
                style:background-color=move || format!("rgb({}, {}, 100)", x.get(), 100)
                style:max-width="400px"
                // Set a CSS variable for stylesheet use
                style=("--columns", x)
            >
                "Click to Move"
            </button>
        </div>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(App);
}
