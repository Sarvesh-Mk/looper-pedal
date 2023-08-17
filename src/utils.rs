#![allow(dead_code)] 
use leptos::*;

#[derive(Clone)]
pub struct RecordingButton {
    count: ReadSignal<i32>,
    set_count: WriteSignal<i32>,
    text: String,
}

impl RecordingButton {
    pub fn new(cx: Scope) -> RecordingButton{
        let (count, set_count) = create_signal(cx, 0);
        RecordingButton{         
            count: count,
            set_count: set_count,
            text: String::from("start recording"),
        }
        
    }


    pub fn click(&mut self) {
        self.set_count.update(|n| {
            if *n == true as i32 {
                *n = false as i32;
            }
            else {
                *n=true as i32; 
            }
        }
        );
    }

    pub fn set_text(self) -> String{
        if self.count.get() == true as i32 {
            "Recording".to_string()
        } else {
            "Start Recording".to_string()
        }
    } 
}