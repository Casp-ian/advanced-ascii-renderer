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

@group(0)
@binding(4)
var<uniform> lineBuffer: array<LinePiece, 5>;

const PI = 3.14159265358979323846264338327950288;

struct Dimensions {
    inputWidth: u32,
    inputHeight: u32,
    outputWidth: u32,
    outputHeight: u32,
}

struct Rotation {
    direction: u32,
}

// TODO ngl this physically hurts me too ;-;
struct LinePiece {
    a: vec4<f32>,
    b: vec4<f32>,
    c: vec4<f32>,
    d: vec4<f32>,
}

// x
// <
// >
// (
// )
// ^
// O
// v

struct PixelData {
    direction: u32,
    color: u32,
    brightness: f32,
}

fn coordsInput(x: u32, y: u32) -> u32 {
    return x + (y * resolutions.inputWidth);
}
fn coordsOutput(x: u32, y: u32) -> u32 {
    return x + (y * resolutions.outputWidth);
}

// TODO alpha influence on brightness
fn brightness(vec: vec3<f32>) -> f32 {
    // return dot(vec, vec3f32>(1.0 / 3.0)); // average
    return dot(vec, vec3<f32>(0.2126, 0.7152, 0.0722));
}

@compute
@workgroup_size(1)
fn do_edges(@builtin(global_invocation_id) global_id: vec3<u32>) {
    if (global_id.x == 0) || (global_id.y == 0) || (global_id.x >= resolutions.inputWidth - 1) || (global_id.y >= resolutions.inputHeight - 1) {
        intermediateBuffer[coordsInput(global_id.x, global_id.y)] = Rotation(0); // none
        return;
    }

    // TODO another 'larger kernel' edge detection to make edges thinner
    // should also maybe be per color
    // dont even understand why the edges are thick right now

    // TODO check if this even has any effect :(    
    // we do sobel for every color, so for example the edge between a green and red area are very visible, and not only between white and black
    // maybe all of this sobel stuff needs to be adjusted for color space now?

    // Sobel
    let gxrgb = (
          1 * unpack4x8unorm( inputTexture[coordsInput(global_id.x - 1, global_id.y - 1)] ).rgb
        + 2 * unpack4x8unorm( inputTexture[coordsInput(global_id.x - 1, global_id.y + 0)] ).rgb
        + 1 * unpack4x8unorm( inputTexture[coordsInput(global_id.x - 1, global_id.y + 1)] ).rgb
        - 1 * unpack4x8unorm( inputTexture[coordsInput(global_id.x + 1, global_id.y - 1)] ).rgb
        - 2 * unpack4x8unorm( inputTexture[coordsInput(global_id.x + 1, global_id.y + 0)] ).rgb
        - 1 * unpack4x8unorm( inputTexture[coordsInput(global_id.x + 1, global_id.y + 1)] ).rgb
    ) / 8;
    let gyrgb = (
          1 * unpack4x8unorm( inputTexture[coordsInput(global_id.x - 1, global_id.y - 1)] ).rgb
        + 2 * unpack4x8unorm( inputTexture[coordsInput(global_id.x + 0, global_id.y - 1)] ).rgb
        + 1 * unpack4x8unorm( inputTexture[coordsInput(global_id.x + 1, global_id.y - 1)] ).rgb
        - 1 * unpack4x8unorm( inputTexture[coordsInput(global_id.x - 1, global_id.y + 1)] ).rgb
        - 2 * unpack4x8unorm( inputTexture[coordsInput(global_id.x + 0, global_id.y + 1)] ).rgb
        - 1 * unpack4x8unorm( inputTexture[coordsInput(global_id.x + 1, global_id.y + 1)] ).rgb
    ) / 8;

    // magnitudes for every color channel
    let ar = sqrt(pow(gxrgb.r, 2.0) + pow(gyrgb.r, 2.0));
    let ag = sqrt(pow(gxrgb.g, 2.0) + pow(gyrgb.g, 2.0));
    let ab = sqrt(pow(gxrgb.b, 2.0) + pow(gyrgb.b, 2.0));

    var gx: f32 = 0.0;
    var gy: f32 = 0.0;

    // TODO rename magnitude threshold
    let al = 0.05;
    if (ar > al && ag > al && ab > al) {
        gx = (gxrgb.r + gxrgb.g + gxrgb.g) / 3;
        gy = (gyrgb.r + gyrgb.g + gyrgb.g) / 3;
    } else if (ar > al && ag > al) {
        // red green
        gx = (gxrgb.r + gxrgb.g) / 2;
        gy = (gyrgb.r + gyrgb.g) / 2;
    } else if (ar > al && ab > al) {
        // red blue
        gx = (gxrgb.r + gxrgb.b) / 2;
        gy = (gyrgb.r + gyrgb.b) / 2;
    } else if (ag > al && ab > al) {
        // red green
        gx = (gxrgb.g + gxrgb.b) / 2;
        gy = (gyrgb.g + gyrgb.b) / 2;
    } else if (ar > al) {
        // red
        gx = gxrgb.r;
        gy = gyrgb.r;
    } else if (ag > al) {
        // green
        gx = gxrgb.g;
        gy = gyrgb.g;
    } else if (ab > al) {
        // blue
        gx = gxrgb.b;
        gy = gyrgb.b;
    } else {
        intermediateBuffer[coordsInput(global_id.x, global_id.y)] = Rotation(0); // none
        return;
    }

    let dir = atan2(gy, gx);
    // grab a picture of a unit circle to make sense of this next part

    if ((dir <= PI / 6.0) && (dir >= -1.0 * PI / 6.0))
        || ((dir >= 5.0 * PI / 6.0) || (dir <= -5.0 * PI / 6.0))
    {
        intermediateBuffer[coordsInput(global_id.x, global_id.y)] = Rotation(1); // '|'
        
    } else if ((dir >= PI / 6.0) && (dir <= PI / 3.0))
        || ((dir >= -5.0 * PI / 6.0) && (dir <= -2.0 * PI / 3.0))
    {
        intermediateBuffer[coordsInput(global_id.x, global_id.y)] = Rotation(2); // '/'
        
    } else if ((dir <= 2.0 * PI / 3.0) && (dir >= PI / 3.0))
        || ((dir >= -2.0 * PI / 3.0) && (dir <= -1.0 * PI / 3.0))
    {
        intermediateBuffer[coordsInput(global_id.x, global_id.y)] = Rotation(3); // '-'
        
    } else if ((dir <= -1.0 * PI / 6.0) && (dir >= -1.0 * PI / 3.0))
        || ((dir <= 5.0 * PI / 6.0) && (dir >= 2.0 * PI / 3.0))
    {
        intermediateBuffer[coordsInput(global_id.x, global_id.y)] = Rotation(4); // '\'
        
    } else {
        // TODO this really should be impossible to happen, but it does, i think the atan2 function returns an error or something and then this happens
        intermediateBuffer[coordsInput(global_id.x, global_id.y)] = Rotation(0); // none
    }

}

@compute
@workgroup_size(1)
fn do_scale(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // square of pixels to take into account when downscaling
    let outsideXL = global_id.x * resolutions.inputWidth / (resolutions.outputWidth + 1);
    let outsideYL = global_id.y * resolutions.inputHeight / (resolutions.outputHeight + 1);
    let outsideXR = ((global_id.x + 1) * resolutions.inputWidth / (resolutions.outputWidth + 1)) - 1;
    let outsideYR = ((global_id.y + 1) * resolutions.inputHeight / (resolutions.outputHeight + 1)) - 1;
    let outsideXC = (outsideXL + outsideXR) / 2;
    let outsideYC = (outsideYL + outsideYR) / 2;

    let direction = intermediateBuffer[coordsInput(outsideXC, outsideYC)].direction;

    // TODO get center pixel or some other downscaling method
    let packedColorPixel = inputTexture[coordsInput(outsideXC, outsideYC)];
    let colorPixel: vec4<f32> = unpack4x8unorm( packedColorPixel );
    // TODO grab more surrounding pixels to scale down better
    // especialy the angles

    let test: f32 = lineBuffer[0].a[0];

    let brightness = brightness(colorPixel.rgb); 

    outputBuffer[coordsOutput(global_id.x, global_id.y)] = PixelData(
        direction,
        packedColorPixel,
        brightness
    );
}
