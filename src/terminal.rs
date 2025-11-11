use crossterm::terminal;

// this kinda shouldnt be in the `terminal` file
pub fn get_scale(
    char_dimensions: (u32, u32),
    specified_output: (Option<u32>, Option<u32>),
    input_scale: (u32, u32),
    output_scale_limit: (u32, u32),
) -> ((u32, u32), (u32, u32)) {
    let aspect_ratio_input = input_scale.1 as f32 / input_scale.0 as f32;
    let aspect_ratio_output = char_dimensions.1 as f32 / char_dimensions.0 as f32; // i dont think this really is aspect ratio, what it is i dont know tho

    let yfromx = |x: u32| {
        return (x as f32 * aspect_ratio_input / aspect_ratio_output) as u32;
    };
    let xfromy = |y: u32| {
        return (y as f32 / aspect_ratio_input * aspect_ratio_output) as u32;
    };

    let output_scale = match specified_output {
        (Some(x), Some(y)) => (x, y),
        (Some(x), None) => (x, yfromx(x)),
        (None, Some(y)) => (xfromy(y), y),
        (None, None) => {
            let x = (output_scale_limit.0, yfromx(output_scale_limit.0));

            if x.1 > output_scale_limit.1 {
                (xfromy(output_scale_limit.1), output_scale_limit.1)
            } else {
                x
            }
        }
    };

    // TODO here we could check if the aspect ration ends up being close to matching the original, and warn if we think the result will be stretched

    let internal_scale = (output_scale.0 * 4, output_scale.1 * 4);
    // let internal_scale = (
    //     output_scale.0 * char_dimensions.0,
    //     output_scale.1 * char_dimensions.1,
    // );

    return (internal_scale, output_scale);
}

pub fn get_terminal_size() -> (u32, u32) {
    if let Ok(size) = terminal::size() {
        return (size.0 as u32, size.1 as u32);
    } else {
        let x = 200;
        let y = 50;
        eprintln!(
            "Could not get width and height from terminal, resorting to hardcoded {} by {}",
            x, y
        );
        return (x, y);
    }
}
