use std::io::Cursor;

use image;
use neon::{prelude::*, types::buffer::TypedArray};

// fn resize_image(mut cx: FunctionContext) -> JsResult<JsBuffer> {
//     // Retrieve image buffer and dimensions from JavaScript arguments
//     let buffer = cx.argument::<JsBuffer>(0)?;
//     let width = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
//     let height = cx.argument::<JsNumber>(2)?.value(&mut cx) as u32;

//     // Convert JS Buffer to a byte slice
//     let image_data: &[u8] = buffer.as_slice(&cx);

//     // Perform image resizing
//     let img = image::load_from_memory(&image_data).expect("Failed to load image from memory");
//     let resized = img.resize(width, height, image::imageops::FilterType::Nearest);
//     let mut resized_buffer = Cursor::new(Vec::new());
//     resized
//         .write_to(&mut resized_buffer, image::ImageOutputFormat::Jpeg(100))
//         .expect("Failed to write image to buffer");
//     let img_data = resized_buffer.into_inner();
//     // Convert the byte vector back to a JS Buffer
//     let js_buffer = JsBuffer::external(&mut cx, img_data);

//     Ok(js_buffer)
// }

// register_module!(mut cx, { cx.export_function("resizeImage", resize_image) });

fn resize_image(mut cx: FunctionContext) -> JsResult<JsPromise> {
    // Retrieve image buffer and dimensions from JavaScript arguments
    let buffer = cx.argument::<JsBuffer>(0)?;
    let width = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
    let height = cx.argument::<JsNumber>(2)?.value(&mut cx) as u32;

    // Convert JS Buffer to a byte slice
    let image_data: &[u8] = buffer.as_slice(&cx);

    // Perform image resizing
    let img = image::load_from_memory(&image_data).expect("Failed to load image from memory");
    let resized = img.resize(width, height, image::imageops::FilterType::Nearest);
    let mut resized_buffer = Cursor::new(Vec::new());
    resized
        .write_to(&mut resized_buffer, image::ImageOutputFormat::Jpeg(100))
        .expect("Failed to write image to buffer");
    let img_data = resized_buffer.into_inner();
    // Convert the byte vector back to a JS Buffer
    let js_buffer = JsBuffer::external(&mut cx, img_data);

    Ok(js_buffer)
}

register_module!(mut cx, { cx.export_function("resizeImage", resize_image) });

// // Implement the Task trait for your ResizeTask
// impl Task for ResizeTask {
//     type Output = Vec<u8>; // Output type
//     type Error = String; // Error type
//     type JsEvent = JsBuffer; // JavaScript event type

//     // Perform the heavy computation in a background thread
//     fn perform(&self) -> Result<Self::Output, Self::Error> {
//         let img = image::load_from_memory(&self.image_data).map_err(|e| e.to_string())?;
//         let resized = img.resize(
//             self.width,
//             self.height,
//             image::imageops::FilterType::Nearest,
//         );

//         let mut resized_buffer = Cursor::new(Vec::new());
//         resized
//             .write_to(&mut resized_buffer, ImageOutputFormat::Png)
//             .map_err(|e| e.to_string())?;
//         Ok(resized_buffer.into_inner())
//     }

//     // Convert the Rust data structure back into a JavaScript value
//     fn complete(
//         self,
//         mut cx: TaskContext,
//         result: Result<Self::Output, Self::Error>,
//     ) -> JsResult<Self::JsEvent> {
//         match result {
//             Ok(buffer) => Ok(JsBuffer::external(&mut cx, buffer)),
//             Err(e) => cx.throw_error(e),
//         }
//     }
// }

// fn resize_image(mut cx: FunctionContext) -> JsResult<JsUndefined> {
//     let buffer = cx.argument::<JsBuffer>(0)?.to_vec(&mut cx)?;
//     let width = cx.argument::<JsNumber>(1)?.value(&mut cx) as u32;
//     let height = cx.argument::<JsNumber>(2)?.value(&mut cx) as u32;

//     let task = ResizeTask {
//         image_data: buffer,
//         width,
//         height,
//     };
//     task.schedule(&mut cx);
//     Ok(cx.undefined())
// }

// register_module!(mut cx, { cx.export_function("resizeImage", resize_image) });
