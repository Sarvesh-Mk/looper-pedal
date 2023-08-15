use leptos::*;


#[component]
fn App(cx: Scope) -> impl IntoView {
    let (count, set_count) = create_signal(cx, 0);

    view! {cx,
        <button
            on:click=move |_| {
                set_count.update(|n| {
                    if *n == true as i32 {
                        *n = false as i32
                    }
                    else {*n=true as i32}
                }
                );
            }
        >
            "record"

        </button>
        
        <p>
            {move || count.get()}
        </p>

    } 
}


fn main(){
    leptos::mount_to_body(|cx| view! { cx, <App/>});
}