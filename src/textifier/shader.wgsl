@group(0)
@binding(0)
var<uniform> resolutions: Dimensions;

@group(0)
@binding(1)
var<storage, read_write> inputTexture: array<u32>; // packed u8 color values

@group(0)
@binding(2)
var<storage, read_write> intermediateBuffer: array<IntermediateData>;

@group(0)
@binding(3)
var<storage, read_write> outputBuffer: array<PixelData>;

@group(0)
@binding(4)
var<uniform> lineBuffer: array<vec4<u32>, 20>;
// 5 char * 4 vecs per char
// this will always be accessed through linePiecePixel();

struct Dimensions {
    inputWidth: u32,
    inputHeight: u32,
    outputWidth: u32,
    outputHeight: u32,
}

struct IntermediateData {
    edge: u32,
    // direction: u32,
}

struct PixelData {
    direction: u32,
    color: u32,
    brightness: f32,
}

fn linePiecePixel(i: u32, x: u32, y: u32) -> f32 {
    // i fucking hate this
    // this is probably right
    // i would explain if i understood
    let a: u32 = (i * 4) + (y / 2);
    let b: u32 = (y % 2) + (x / 4);
    let c: u32 = x % 4;
    return unpack4x8unorm(lineBuffer[a][b])[c];
}

fn coordsInput(x: u32, y: u32) -> u32 {
    return x + (y * resolutions.inputWidth);
}
fn coordsOutput(x: u32, y: u32) -> u32 {
    return x + (y * resolutions.outputWidth);
}

fn brightness(vec: vec4<f32>) -> f32 {
    return dot(vec.rgb, vec3<f32>(0.2126, 0.7152, 0.0722)) * vec.a;
}

fn mdot(left: mat3x3<f32>, right: mat3x3<f32>) -> f32 {
    return dot(left[0], right[0]) + dot(left[1], right[1]) + dot(left[2], right[2]);
}

@compute
@workgroup_size(1)
fn do_edges(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // cant do edge detection on the outer edges of the image
    if (global_id.x == 0) || (global_id.y == 0) || (global_id.x >= resolutions.inputWidth - 1) || (global_id.y >= resolutions.inputHeight - 1) {
        intermediateBuffer[coordsInput(global_id.x, global_id.y)] = IntermediateData(0);
        return;
    }

    // NOTE: there is a small advantage to splitting edge detection to collors instead of grayscaling, i think stevehanov just picked bad examples
    // source: https://stevehanov.ca/blog/?id=62

    // TODO make edge detection size scale with proportion to output size

    let kernel = mat3x3f(
        vec3f(
            brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x - 1, global_id.y - 1)] )),
            brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x - 0, global_id.y - 1)] )),
            brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x + 1, global_id.y - 1)] ))
        ),
        vec3f(
            brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x - 1, global_id.y + 0)] )),
            0.0,
            brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x + 1, global_id.y + 0)] ))
        ),
        vec3f(
            brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x - 1, global_id.y + 1)] )),
            brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x - 0, global_id.y + 1)] )),
            brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x + 1, global_id.y + 1)] ))
        )
    );
    
    let sobelX = mat3x3f(
        -1, 0, 1,
        -2, 0, 2,
        -1, 0, 1,
    );
    let sobelY = mat3x3f(
        -1, -2, -1,
        0, 0, 0,
        1, 2, 1,
    );

    var gx: f32 = mdot(kernel, sobelX);
    var gy: f32 = mdot(kernel, sobelY);


    if ((gx * gx + gy * gy) > (0.7 * 0.7)) {
        intermediateBuffer[coordsInput(global_id.x, global_id.y)] = IntermediateData(1); // edge
    } else {
        intermediateBuffer[coordsInput(global_id.x, global_id.y)] = IntermediateData(0); // no edge
    }
}

@compute
@workgroup_size(1)
fn do_scale(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // square of pixels to take into account when downscaling
    let XL = global_id.x * resolutions.inputWidth / (resolutions.outputWidth + 1);
    let YL = global_id.y * resolutions.inputHeight / (resolutions.outputHeight + 1);
    let XR = ((global_id.x + 1) * resolutions.inputWidth / (resolutions.outputWidth + 1)) - 1;
    let YR = ((global_id.y + 1) * resolutions.inputHeight / (resolutions.outputHeight + 1)) - 1;

    var counts: array<u32, 5> = array(0u, 0u, 0u, 0u, 0u);
    for (var y = YL; y <= YR; y++) {
        for (var x = XL; x <= XR; x++) {
            // if both actual pixel, and linepiece pixel are edges, then add that linepiece score
            if (intermediateBuffer[coordsInput(x, y)].edge == 1) {

                let lineX = ((x - XL) * 8) / (XR - XL);
                let lineY = ((y - YL) * 8) / (YR - YL);

                for (var i: u32 = 0u; i < 5u; i++) {

                    if (linePiecePixel(i, lineX, lineY) > 0.0) {
                        counts[i]++;
                    }

                }
            }
        }
    }

    // TODO configurable through uniform
    // let lineThreshold: u32 = resolutions.outputWidth * resolutions.outputHeight / 1000u;
    let lineThreshold: u32 = 0u;

    var direction: u32 = 0u;
    var threshold: u32 = lineThreshold;
    
    for (var i: u32 = 0u; i < 5u; i++) {
        if (counts[i] > threshold) {
            direction = i + 1;
            threshold = counts[i];
        }
    }
    
    let XC = (XL + XR) / 2;
    let YC = (YL + YR) / 2;
    
    let packedColorPixel = inputTexture[coordsInput(XC, YC)];
    let colorPixel: vec4<f32> = unpack4x8unorm( packedColorPixel );

    let brightness = brightness(colorPixel); 

    outputBuffer[coordsOutput(global_id.x, global_id.y)] = PixelData(
        direction,
        packedColorPixel,
        brightness
    );
}
