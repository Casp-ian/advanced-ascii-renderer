@group(0)
@binding(0)
var<uniform> resolutions: Dimensions;

@group(0) @binding(1)
var t_diffuse: texture_2d<f32>;
@group(0) @binding(2)
var s_diffuse: sampler;

// var<storage, read_write> inputTexture: array<u32>; // packed u8 color values

@group(0)
@binding(3)
var<storage, read_write> intermediateBuffer: array<IntermediateData>;

@group(0)
@binding(4)
var<storage, read_write> outputBuffer: array<PixelData>;

@group(0)
@binding(5)
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
    edge: f32,
}

struct PixelData {
    direction: u32,
    color: u32,
    brightness: f32,
}

fn linePiecePixel(i: u32, x: u32, y: u32) -> f32 {
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

    // NOTE: there is a small advantage to splitting edge detection to collors instead of grayscaling, i think stevehanov just picked bad examples to show it off
    // source: https://stevehanov.ca/blog/?id=62

    // TODO make edge detection size scale with proportion to output size

    let stepX = u32(ceil((f32(resolutions.inputWidth) / f32(resolutions.outputWidth)) / 6.0)); // magic number for now
    let stepY = u32(ceil((f32(resolutions.inputHeight) / f32(resolutions.outputHeight)) / 6.0)); // magic number for now
    // let stepX = 1u;
    // let stepY = 1u;

    let kernel = mat3x3f(
        vec3f(
            brightness(textureSampleLevel(t_diffuse, s_diffuse, vec2f(f32(global_id.x - stepX), f32(global_id.y - stepY) ), 1.0 )),
            brightness(textureSampleLevel(t_diffuse, s_diffuse, vec2f(f32(global_id.x - 0    ), f32(global_id.y - stepY) ), 1.0 )),
            brightness(textureSampleLevel(t_diffuse, s_diffuse, vec2f(f32(global_id.x + stepX), f32(global_id.y - stepY) ), 1.0 )),
        ),
        vec3f(
            brightness(textureSampleLevel(t_diffuse, s_diffuse, vec2f(f32(global_id.x - stepX), f32(global_id.y + 0    ) ), 1.0 )),
            0.0,
            brightness(textureSampleLevel(t_diffuse, s_diffuse, vec2f(f32(global_id.x + stepX), f32(global_id.y + 0    ) ), 1.0 )),
        ),
        vec3f(
            brightness(textureSampleLevel(t_diffuse, s_diffuse, vec2f(f32(global_id.x - stepX), f32(global_id.y + stepY) ), 1.0 )),
            brightness(textureSampleLevel(t_diffuse, s_diffuse, vec2f(f32(global_id.x - 0    ), f32(global_id.y + stepY) ), 1.0 )),
            brightness(textureSampleLevel(t_diffuse, s_diffuse, vec2f(f32(global_id.x + stepX), f32(global_id.y + stepY) ), 1.0 )),
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

    var magnitude: f32 = sqrt(gx * gx + gy * gy);

    if (magnitude > 0.8) {
        magnitude = 1.0;
    } else {
        magnitude = 0.0;
    }

    intermediateBuffer[coordsInput(global_id.x, global_id.y)] = IntermediateData(magnitude);
}

@compute
@workgroup_size(1)
fn do_scale(@builtin(global_invocation_id) global_id: vec3<u32>) {
    // square of pixels to take into account when downscaling
    let XL = global_id.x * resolutions.inputWidth / (resolutions.outputWidth + 1);
    let YL = global_id.y * resolutions.inputHeight / (resolutions.outputHeight + 1);
    let XR = ((global_id.x + 1) * resolutions.inputWidth / (resolutions.outputWidth + 1)) - 1;
    let YR = ((global_id.y + 1) * resolutions.inputHeight / (resolutions.outputHeight + 1)) - 1;

    var scores: array<f32, 5> = array(0.0, 0.0, 0.0, 0.0, 0.0);
    for (var y = YL; y <= YR; y++) {
        for (var x = XL; x <= XR; x++) {
            // if both actual pixel, and linepiece pixel are edges, then add that linepiece score

            let edge: f32 = intermediateBuffer[coordsInput(x, y)].edge;
            // if (edge > 0.7) {

            let lineX = ((x - XL) * 8) / (XR - XL);
            let lineY = ((y - YL) * 8) / (YR - YL);

            for (var i: u32 = 0u; i < 5u; i++) {

                let newScore: f32 = edge * linePiecePixel(i, lineX, lineY);
                let fraction: f32 = 1.0 / f32((XR - XL) * (YR - YL));

                scores[i] += newScore;
                scores[i] -= fraction * 10;
            }
        }
    }

    // TODO configurable through uniform
    // let lineThreshold: u32 = resolutions.outputWidth * resolutions.outputHeight / 1000u;
    let lineThreshold: f32 = 0.0;

    var direction: u32 = 0u;
    var threshold: f32 = lineThreshold;
    
    for (var i: u32 = 0u; i < 5u; i++) {
        if (scores[i] > threshold) {
            direction = i + 1;
            threshold = scores[i];
        }
    }
    
    let XC = (XL + XR) / 2;
    let YC = (YL + YR) / 2;
    
    let packedColorPixel =  pack4x8unorm(textureSampleLevel(t_diffuse, s_diffuse, vec2f(f32(XC) / f32(resolutions.inputWidth), f32(YC) / f32(resolutions.inputHeight)), 1.0));
    let colorPixel: vec4<f32> = unpack4x8unorm( packedColorPixel );

    let brightness = brightness(colorPixel); 

    outputBuffer[coordsOutput(global_id.x, global_id.y)] = PixelData(
        direction,
        packedColorPixel,
        brightness
    );
}
