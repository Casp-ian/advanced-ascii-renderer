@group(0)
@binding(0)
var<uniform> resolutions: Dimensions;

@group(0)
@binding(1)
var inputTexture: texture_2d<f32>;

@group(0)
@binding(2)
var<storage, read_write> intermediateBuffer: array<Test>;

@group(0)
@binding(3)
var<storage, read_write> outputBuffer: array<PixelData>;

struct Dimensions {
    inputWidth: u32,
    inputHeight: u32,
    outputWidth: u32,
    outputHeight: u32,
}
struct Test {
    gx: f32,
    gy: f32,
}

struct PixelData { // TODO rename here and in image.rs
    gx: f32,
    gy: f32,
    r: f32,
    g: f32,
    b: f32,
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
          1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y - 1), 0).rgb)
        + 2 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y + 0), 0).rgb)
        + 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y + 1), 0).rgb)
        - 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y - 1), 0).rgb)
        - 2 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y + 0), 0).rgb)
        - 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y + 1), 0).rgb)
    );
    let gy = (
          1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y - 1), 0).rgb)
        + 2 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 0, global_id.y - 1), 0).rgb)
        + 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y - 1), 0).rgb)
        - 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x - 1, global_id.y + 1), 0).rgb)
        - 2 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 0, global_id.y + 1), 0).rgb)
        - 1 * average(textureLoad(inputTexture, vec2<u32>(global_id.x + 1, global_id.y + 1), 0).rgb)
    );

    // TODO, how do i improve the soble edge detection, could use DoG, or some other approach making use of our weird scaling situation

    // textureStore(outputBuffer, global_id.xy, vec4<f32>(gx, gy, 0.0, 0.0));
    intermediateBuffer[coordsInput(global_id.x, global_id.y)] = Test(gx, gy);
}

@compute
@workgroup_size(1)
fn do_scale(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let outsideX = global_id.x * resolutions.inputWidth / resolutions.outputWidth;
    let outsideY = global_id.y * resolutions.inputHeight / resolutions.outputHeight;

    let colorPixel = textureLoad(inputTexture, vec2<u32>(outsideX, outsideY), 0);
    let intermediatePixel = intermediateBuffer[coordsInput(outsideX, outsideY)];

    // TODO alpha influence?
    let brightness = (colorPixel.r * 0.2126) + (colorPixel.g * 0.7152) + (colorPixel.b * 0.0722); 

    outputBuffer[coordsOutput(global_id.x, global_id.y)] = PixelData(
        intermediatePixel.gx,
        intermediatePixel.gy,
        colorPixel.r,
        colorPixel.g,
        colorPixel.b,
        brightness
    );
}
