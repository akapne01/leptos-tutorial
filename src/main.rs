use leptos::*;

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

    /*
        defines user interfaces using a JSX-like format via the view macro.
     */
    view! {
        /*
            Defines a click event listener
            Closures are used to tell the leptos framework that this is something
            that can change.
            On click a targeted update is made, only changing this one thing, nothing else.
            Only functions are reactive!!!
            {count} passes in a function, telling the framework to update the view every time 
            count changes. 
            {count()} accesses the value of count once, and passes an i32 into the view, 
            rendering it once, unreactively.
            You can see here that while set_count just sets the value, set_count.update() 
            gives us a mutable reference and mutates the value in place. Either one will 
            trigger a reactive update in our UI.
         */
        <button on:click=move |_| {
            set_count.update(|n| *n += 1);
        }>"Click Me: "{move || count.get()}
        </button>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
