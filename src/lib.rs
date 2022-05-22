use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use web_sys::CanvasRenderingContext2d;
use web_sys::InputEvent;
use web_sys::HtmlInputElement;
use web_sys::HtmlCanvasElement;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[wasm_bindgen]
pub fn start() {
    let document = web_sys::window().unwrap().document().unwrap();

    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    
    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    let slider = HtmlInputElement::from(JsValue::from(document.get_element_by_id("myRange").unwrap()));
   
    {
    let context_clone = context.clone();
    let slider_clone = slider.clone();
    let closure = Closure::wrap(Box::new(move |_: InputEvent| {
        let slider_value: i32 = slider_clone.value().parse().unwrap();
        log(&format!("{}", slider_value));

        context_clone.clear_rect(0.0,0.0, 400.0, 400.0);
        draw_table(&context_clone);

        context_clone.begin_path();
        context_clone.set_line_width(3.0);
        context_clone.rect(2.5,2.5, limit_width(f64::from(slider_value) - 2.5), 400.0 - 5.0);
        let color = JsValue::from_str("rgba(255, 0, 0, 0.7)");
        context_clone.set_fill_style(&color);
        context_clone.set_image_smoothing_enabled(false);
        context_clone.fill();


    }) as Box<dyn FnMut(_)>);

    Some(slider.add_event_listener_with_callback("input", &closure.as_ref().unchecked_ref()));
    closure.forget();
    }
    
    draw_table(&context);
}

fn draw_table(context: &CanvasRenderingContext2d)
{
    let w: u32 = 400; //canvas.width();
    let h: u32 = 400; //canvas.height();

    let n = 10; // number of rows
    let  m = 10; // number of columns

    let w_block = w / m;
    let h_block = h / n;

    //log(&format!("width of block: {}\nheight of block: {}", w_block, h_block));

    context.begin_path();
    context.set_line_width(1.0);
    context.set_image_smoothing_enabled(false);

    for i in 0..n 
    {
        for j in 0..m 
        {
            if i % 2 == 0 
            {
                if j % 2 == 0  
                {
                    context.rect((j*w_block).into(), (i * h_block).into(), w_block.into(), h_block.into());  
                } 
            }
            else 
            {
                if j % 2 == 1  
                {
                    context.rect((j*w_block).into(), (i * h_block).into(), w_block.into(), h_block.into());  
                }   
            }
        }
    }
    
    let color = JsValue::from_str("gray");
    context.set_stroke_style(&color);
    context.stroke();

    context.begin_path();
    context.set_image_smoothing_enabled(false);
    let color = JsValue::from_str("black");
    context.set_stroke_style(&color);
    context.set_line_width(5.0);
    context.rect(0.0, 0.0, w.into(), h.into());
    context.stroke();
}

fn limit_width(width: f64) -> f64
{
    if width > 400.0 - 5.0 
    {
        return 400.0 - 5.0;
    }
    else if width < 0.0 
    {
        return 0.0;
    }
    return width;
}
