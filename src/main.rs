use leptos::*;
mod utils;

#[component]
fn App(cx: Scope) -> impl IntoView {
    let btn1 = utils::RecordingButton::new(cx);
    let (count, set_count) = create_signal(cx, 0);

    view! {cx,
        <button
            on:click= |_| {
                let btn_ref = &btn1;
                btn_ref.click()
            }
        >
            {
                btn1.clone().set_text()
            }

        </button>
        <button
            on:click=move |_| {
                set_count.update(|n| {
                    if *n == true as i32 {
                        *n = false as i32;
                    }
                    else {
                        *n=true as i32; 
                    }
                }
                );
            }
        >
            {move || {
                if count.get() == true as i32 {
                    "Recording"
                } else {
                    "Start Recording"
                }
            }
            }

        </button>
        
        <p>
            {move || count.get()}
        </p>

    } 
}


fn main(){
    leptos::mount_to_body(|cx| view! { cx, <App/>});
}