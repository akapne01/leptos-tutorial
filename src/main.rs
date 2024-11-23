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
    let (x, set_x) = create_signal(0);

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
        }
        // the class: syntax reactively updates a single class
        // here, we'll set the `red` class when `count` is odd
        class:red=move || count.get() % 2 == 1
        >"Click Me: "{move || count.get()}
        </button>

        <progress
        max="50"
        // signals are functions, so `value=count` and `value=move || count.get()`
        // are interchangeable.
        value=count
    />

        <button
            on:click={move |_| {
                set_x.update(|n| *n += 10);
            }}
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
        
       
    
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(|| view! { <App/> })
}
