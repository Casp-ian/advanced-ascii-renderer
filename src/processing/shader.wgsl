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
    gx: f32,
    gy: f32,
}

struct PixelData { // TODO rename here and in image.rs
    gx: f32,
    gy: f32,
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

@compute
@workgroup_size(1)
fn do_edges(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if (global_id.x == 0) || (global_id.y == 0) {
        return;
    }
    if (global_id.x >= resolutions.inputWidth - 1) || (global_id.y >= resolutions.inputHeight - 1) {
        return;
    }
            
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

    // TODO, how do i improve the soble edge detection, could use DoG, or some other approach making use of our weird scaling situation

    // textureStore(outputBuffer, global_id.xy, vec4<f32>(gx, gy, 0.0, 0.0));
    intermediateBuffer[coordsInput(global_id.x, global_id.y)] = Rotation(gx, gy);
}

@compute
@workgroup_size(1)
fn do_scale(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let outsideX = global_id.x * resolutions.inputWidth / resolutions.outputWidth;
    let outsideY = global_id.y * resolutions.inputHeight / resolutions.outputHeight;

    let packedColorPixel = inputTexture[coordsInput(outsideX, outsideY)];
    let colorPixel: vec4<f32> = unpack4x8unorm( packedColorPixel );
    let intermediatePixel = intermediateBuffer[coordsInput(outsideX, outsideY)];

    // TODO alpha influence?
    let brightness = (colorPixel.r * 0.2126) + (colorPixel.g * 0.7152) + (colorPixel.b * 0.0722); 

    outputBuffer[coordsOutput(global_id.x, global_id.y)] = PixelData(
        intermediatePixel.gx,
        intermediatePixel.gy,
        packedColorPixel,
        brightness
    );
}
