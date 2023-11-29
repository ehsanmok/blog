use neon::prelude::*;

use anyhow;
use candle::DType;
use candle_core as candle;
use candle_nn::VarBuilder;
use candle_transformers::models::segment_anything::sam;

fn generate_sam(mut cx: FunctionContext) -> JsResult<JsString> {
    let image_path = cx.argument::<JsString>(0)?.value(&mut cx);
    let points = cx.argument::<JsArray>(1)?;
    let neg_points = cx.argument::<JsArray>(2)?;
    let points = get_points(&mut cx, points);
    let neg_points = get_points(&mut cx, neg_points);
    _generate_sam(image_path, points, neg_points).expect("error generating sam");
    Ok(cx.string("ok"))
}

fn get_points(cx: &mut FunctionContext, handle: Handle<JsArray>) -> Vec<String> {
    let points = handle.to_vec(cx).expect("error converting to vec");
    let mut ret: Vec<String> = Vec::new();
    for point in points {
        let point_string = point
            .downcast::<JsString, FunctionContext>(cx)
            .or_else(|_| cx.throw_error("Array element is not a string"))
            .unwrap()
            .value(cx);
        ret.push(point_string);
    }
    // return as a vec but a single contiguous string
    ret = vec![ret.join(",")];
    ret
}

fn _generate_sam(
    image_path: String,
    points: Vec<String>,
    neg_points: Vec<String>,
) -> anyhow::Result<()> {
    let device = candle_examples::device(true)?; // use CPU
    let (image, initial_h, initial_w) =
        candle_examples::load_image(&image_path, Some(sam::IMAGE_SIZE))?;
    let image = image.to_device(&device)?;
    println!("loaded image {image:?}");
    let api = hf_hub::api::sync::Api::new()?;
    let api = api.model("lmz/candle-sam".to_string());
    let filename = "mobile_sam-tiny-vitt.safetensors";
    let model = api.get(filename)?;
    let vb = unsafe { VarBuilder::from_mmaped_safetensors(&[model], DType::F32, &device)? };
    let sam = sam::Sam::new_tiny(vb)?;

    // Default options similar to the Python version.
    let bboxes = sam.generate_masks(
        &image,
        /* points_per_side */ 32,
        /* crop_n_layer */ 0,
        /* crop_overlap_ratio */ 512. / 1500.,
        /* crop_n_points_downscale_factor */ 1,
    )?;
    for (idx, bbox) in bboxes.iter().enumerate() {
        println!("{idx} {bbox:?}");
        let mask = (&bbox.data.to_dtype(DType::U8)? * 255.)?;
        let (h, w) = mask.dims2()?;
        let mask = mask.broadcast_as((3, h, w))?;
        candle_examples::save_image_resize(
            &mask,
            format!("sam_mask{idx}.png"),
            initial_h,
            initial_w,
        )?;
    }
    let iter_points = points.iter().map(|p| (p, true));
    let iter_neg_points = neg_points.iter().map(|p| (p, false));
    let points = iter_points
        .chain(iter_neg_points)
        .map(|(point, b)| {
            use std::str::FromStr;
            let xy = point.split(',').collect::<Vec<_>>();
            if xy.len() != 2 {
                anyhow::bail!("expected format for points is 0.4,0.2")
            }
            Ok((f64::from_str(xy[0])?, f64::from_str(xy[1])?, b))
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let start_time = std::time::Instant::now();
    let (mask, iou_predictions) = sam.forward(&image, &points, false)?;
    println!(
        "mask generated in {:.2}s",
        start_time.elapsed().as_secs_f32()
    );
    println!("mask:\n{mask}");
    println!("iou_predictions: {iou_predictions}");

    let mask = (mask.ge(0.)? * 255.)?;
    let (_one, h, w) = mask.dims3()?;
    let mask = mask.expand((3, h, w))?;

    let mut img = image::io::Reader::open(&image_path)?
        .decode()
        .map_err(candle::Error::wrap)?;
    let mask_pixels = mask.permute((1, 2, 0))?.flatten_all()?.to_vec1::<u8>()?;
    let mask_img: image::ImageBuffer<image::Rgb<u8>, Vec<u8>> =
        match image::ImageBuffer::from_raw(w as u32, h as u32, mask_pixels) {
            Some(image) => image,
            None => anyhow::bail!("error saving merged image"),
        };
    let mask_img = image::DynamicImage::from(mask_img).resize_to_fill(
        img.width(),
        img.height(),
        image::imageops::FilterType::CatmullRom,
    );
    for x in 0..img.width() {
        for y in 0..img.height() {
            let mask_p = imageproc::drawing::Canvas::get_pixel(&mask_img, x, y);
            if mask_p.0[0] > 100 {
                let mut img_p = imageproc::drawing::Canvas::get_pixel(&img, x, y);
                img_p.0[2] = 255 - (255 - img_p.0[2]) / 2;
                img_p.0[1] /= 2;
                img_p.0[0] /= 2;
                imageproc::drawing::Canvas::draw_pixel(&mut img, x, y, img_p)
            }
        }
    }
    for (x, y, b) in points {
        let x = (x * img.width() as f64) as i32;
        let y = (y * img.height() as f64) as i32;
        let color = if b {
            image::Rgba([255, 0, 0, 200])
        } else {
            image::Rgba([0, 255, 0, 200])
        };
        imageproc::drawing::draw_filled_circle_mut(&mut img, (x, y), 3, color);
    }
    img.save("sam_merged.jpg")?;
    Ok(())
}

register_module!(mut cx, { cx.export_function("generateSam", generate_sam) });
