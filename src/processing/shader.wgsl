@group(0)
@binding(0)
var<uniform> resolutions: Dimensions;

@group(0)
@binding(1)
var<storage, read_write> inputTexture: array<u32>; // packed u8 color values

@group(0)
@binding(2)
var<storage, read_write> intermediateBuffer: array<Rotation>;

@group(0)
@binding(3)
var<storage, read_write> outputBuffer: array<PixelData>;

struct Dimensions {
    inputWidth: u32,
    inputHeight: u32,
    outputWidth: u32,
    outputHeight: u32,
}

struct Rotation {
    direction: u32,
}
// we dont have enums in wgsl yet
// 0: None
// 1: |
// 2: /
// 3: -
// 4: \

struct PixelData {
    direction: u32,
    // gy: f32,
    color: u32,
    brightness: f32,
}


fn coordsInput(x: u32, y: u32) -> u32 {
    return x + (y * resolutions.inputWidth);
}
fn coordsOutput(x: u32, y: u32) -> u32 {
    return x + (y * resolutions.outputWidth);
}

fn average(vec: vec3<f32>) -> f32 {
    return dot(vec, vec3<f32>(1.0 / 3.0)); // Efficient averaging using the dot product
}

const PI = 3.14159265358979323846264338327950288;

@compute
@workgroup_size(1)
fn do_edges(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if (global_id.x == 0) || (global_id.y == 0) {
        return;
    }
    if (global_id.x >= resolutions.inputWidth - 1) || (global_id.y >= resolutions.inputHeight - 1) {
        return;
    }

    // TODO move more cpu stuff into here
    
    let gx = (
          1 * average( unpack4x8unorm( inputTexture[coordsInput(global_id.x - 1, global_id.y - 1)] ).rgb )
        + 2 * average( unpack4x8unorm( inputTexture[coordsInput(global_id.x - 1, global_id.y + 0)] ).rgb )
        + 1 * average( unpack4x8unorm( inputTexture[coordsInput(global_id.x - 1, global_id.y + 1)] ).rgb )
        - 1 * average( unpack4x8unorm( inputTexture[coordsInput(global_id.x + 1, global_id.y - 1)] ).rgb )
        - 2 * average( unpack4x8unorm( inputTexture[coordsInput(global_id.x + 1, global_id.y + 0)] ).rgb )
        - 1 * average( unpack4x8unorm( inputTexture[coordsInput(global_id.x + 1, global_id.y + 1)] ).rgb )
    );
    let gy = (
          1 * average( unpack4x8unorm( inputTexture[coordsInput(global_id.x - 1, global_id.y - 1)] ).rgb )
        + 2 * average( unpack4x8unorm( inputTexture[coordsInput(global_id.x + 0, global_id.y - 1)] ).rgb )
        + 1 * average( unpack4x8unorm( inputTexture[coordsInput(global_id.x + 1, global_id.y - 1)] ).rgb )
        - 1 * average( unpack4x8unorm( inputTexture[coordsInput(global_id.x - 1, global_id.y + 1)] ).rgb )
        - 2 * average( unpack4x8unorm( inputTexture[coordsInput(global_id.x + 0, global_id.y + 1)] ).rgb )
        - 1 * average( unpack4x8unorm( inputTexture[coordsInput(global_id.x + 1, global_id.y + 1)] ).rgb )
    );

    let magnitude_threshold = 0.8;
    let magnitude = sqrt(pow(gx, 2.0) + pow(gy, 2.0));
    let dir = atan2(gy, gx);

    if magnitude > magnitude_threshold {
        if (dir < (2.0 * PI / 3.0) && dir > (PI / 3.0))
            || (dir < (-2.0 * PI / 3.0) && dir > (-1.0 * PI / 3.0))
        {
            // direction = Direction::LeftToRight;
            intermediateBuffer[coordsInput(global_id.x, global_id.y)] = Rotation(3);
        } else if ((dir < PI / 6.0) && (dir > -1.0 * PI / 6.0))
            || ((dir > 5.0 * PI / 6.0) || (dir < -5.0 * PI / 6.0))
        {
            // direction = Direction::TopToBottom;
            intermediateBuffer[coordsInput(global_id.x, global_id.y)] = Rotation(1);
        } else if ((dir > PI / 6.0) && (dir < PI / 3.0))
            || ((dir > -5.0 * PI / 6.0) && (dir < -2.0 * PI / 3.0))
        {
            // direction = Direction::ToprightToBotleft;
            intermediateBuffer[coordsInput(global_id.x, global_id.y)] = Rotation(2);
        } else if ((dir < -1.0 * PI / 6.0) && (dir > -1.0 * PI / 3.0))
            || ((dir < 5.0 * PI / 6.0) && (dir > 2.0 * PI / 3.0))
        {
            // direction = Direction::TopleftToBotright;
            intermediateBuffer[coordsInput(global_id.x, global_id.y)] = Rotation(4);
        } else {
            // direction = Direction::None;
            intermediateBuffer[coordsInput(global_id.x, global_id.y)] = Rotation(0);
        }
    } else {
        // direction = Direction::None;
        intermediateBuffer[coordsInput(global_id.x, global_id.y)] = Rotation(0);
    }

}

@compute
@workgroup_size(1)
fn do_scale(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // square of pixels to take into account when downscaling
    // let outsideXL = global_id.x * resolutions.inputWidth / (resolutions.outputWidth + 1);
    // let outsideYL = global_id.y * resolutions.inputHeight / (resolutions.outputHeight + 1);
    // let outsideXR = ((global_id.x + 1) * resolutions.inputWidth / (resolutions.outputWidth + 1)) - 1;
    // let outsideYR = ((global_id.y + 1) * resolutions.inputHeight / (resolutions.outputHeight + 1)) - 1;

    let outsideX = global_id.x * resolutions.inputWidth / resolutions.outputWidth;
    let outsideY = global_id.y * resolutions.inputHeight / resolutions.outputHeight;

    let packedColorPixel = inputTexture[coordsInput(outsideX, outsideY)];
    let colorPixel: vec4<f32> = unpack4x8unorm( packedColorPixel );
    let intermediatePixel = intermediateBuffer[coordsInput(outsideX, outsideY)];
    // TODO grab more surrounding pixels to scale down better
    // especialy the angles


    // TODO alpha influence?
    let brightness = (colorPixel.r * 0.2126) + (colorPixel.g * 0.7152) + (colorPixel.b * 0.0722); 

    outputBuffer[coordsOutput(global_id.x, global_id.y)] = PixelData(
        intermediatePixel.direction,
        packedColorPixel,
        brightness
    );
}
