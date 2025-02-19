use crossterm::terminal;

pub fn get_cols_and_rows(
    char_width: u32,
    char_height: u32,
    optional_columns: Option<u32>,
    optional_rows: Option<u32>,
    image_width: u32,
    image_height: u32,
) -> (u32, u32) {
    let (columns, rows) = match (optional_columns, optional_rows) {
        (Some(x), Some(y)) => {
            eprintln!(
                "you specified both image collumns and rows, image aspect ratio might be messed up"
            );
            return (x, y);
        }
        (Some(x), None) => (
            x,
            calculate_other_side_by_aspect(x, char_width, char_height, image_width, image_height),
        ),
        (None, Some(y)) => (
            calculate_other_side_by_aspect(y, char_height, char_width, image_height, image_width),
            y,
        ),
        (None, None) => get_fitting_terminal(char_width, char_height, image_width, image_height),
    };

    if image_width / columns < 8 || image_height / rows < 8 {
        // NOTE when we switch to a font reading method this will no longer be an issue
        eprintln!("lines could very well be fucked");
    }

    return (columns, rows);
}

pub fn calculate_other_side_by_aspect(
    x: u32,
    source_aspect_x: u32,
    source_aspect_y: u32,
    target_aspect_x: u32,
    target_aspect_y: u32,
) -> u32 {
    (x as f32 * (target_aspect_y as f32 / source_aspect_y as f32)
        / (target_aspect_x as f32 / source_aspect_x as f32))
        .floor() as u32 //floor or round?
}

pub fn get_fitting_terminal(
    char_width: u32,
    char_height: u32,
    image_width: u32,
    image_height: u32,
) -> (u32, u32) {
    let max_terminal_chars_x: u32;
    let max_terminal_chars_y: u32;

    if let Ok(size) = terminal::size() {
        max_terminal_chars_x = size.0 as u32;
        max_terminal_chars_y = size.1 as u32;
    } else {
        max_terminal_chars_x = 200;
        max_terminal_chars_y = 50;
        eprintln!(
            "Could not get width and height from terminal, resorting to hardcoded {} by {}",
            max_terminal_chars_x, max_terminal_chars_y
        );
    }

    let y_chars = calculate_other_side_by_aspect(
        max_terminal_chars_x,
        char_width,
        char_height,
        image_width,
        image_height,
    );

    if y_chars <= max_terminal_chars_y {
        return (max_terminal_chars_x, y_chars);
    }

    let x_chars = calculate_other_side_by_aspect(
        max_terminal_chars_y,
        char_height,
        char_width,
        image_height,
        image_width,
    );
    return (x_chars, max_terminal_chars_y);
}
