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
// we dont have enums in wgsl yet
// 0: None
// 1: |
// 2: /
// 3: -
// 4: \

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

fn average(vec: vec3<f32>) -> f32 {
    // return dot(vec, vec3<f32>(1.0 / 3.0)); // averaging using the dot product
    return (vec.r * 0.2126) + (vec.g * 0.7152) + (vec.b * 0.0722); 
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
    let al = 0.15;
    if (ar > al && ag > al && ab > al) {
        // average all
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

    var counts = array<u32, 5>(0, 0, 0, 0, 0);
    for (var i: u32 = outsideXL; i <= outsideXR; i++) {
        for (var j: u32 = outsideYL; j <= outsideYR; j++) {
            counts[intermediateBuffer[coordsInput(i, j)].direction]++;
        }
    }
    
    // 1/50th of pixels in a square
    let partOfPixels: u32 = ((outsideXR - outsideXL) + (outsideYR - outsideYL)) / 50;

    var maxIndex: u32 = 0;
    // var maxCount: u32 = partOfPixels; // this is the doorstep, if there are more than this amount of edge pixels it will be an edge
    var maxCount: u32 = 0;
    for (var i: u32 = 1; i < 5; i++) {
        if (counts[i] > maxCount) {
            maxIndex = i;
            maxCount = counts[i];
        }
    }
    
    let direction = maxIndex;

    // TODO get center pixel or some other downscaling method
    let packedColorPixel = inputTexture[coordsInput(outsideXL, outsideYL)];
    let colorPixel: vec4<f32> = unpack4x8unorm( packedColorPixel );
    // TODO grab more surrounding pixels to scale down better
    // especialy the angles


    // TODO alpha influence?
    // let brightness = (colorPixel.r * 0.2126) + (colorPixel.g * 0.7152) + (colorPixel.b * 0.0722); 
    let brightness = average(colorPixel.rgb); 

    outputBuffer[coordsOutput(global_id.x, global_id.y)] = PixelData(
        direction,
        packedColorPixel,
        brightness
    );
}
