@group(0)
@binding(0)
var<uniform> config: ConfigData;

@group(0)
@binding(1)
var<storage, read_write> inputTexture: array<u32>; // packed u8 color values

@group(0)
@binding(2)
var<storage, read_write> intermediateBuffer: array<u32>; // packed f16 magnitude and angle

@group(0)
@binding(3)
var<storage, read_write> outputBuffer: array<PixelData>;

struct ConfigData {
    inputWidth: u32,
    inputHeight: u32,
    outputWidth: u32,
    outputHeight: u32,
    threshold: f32,
}

struct PixelData {
    direction: u32,
    color: u32,
    brightness: f32,
}

fn coordsInput(x: u32, y: u32) -> u32 {
    // NOTE: right now this is just repeat, it should be mirrored repeat
    let a: u32 = x % config.inputWidth;
    let b: u32 = y % config.inputHeight;
    return a + (b * config.inputWidth);
}

fn coordsOutput(x: u32, y: u32) -> u32 {
    // i dont think we want this to wrap
    // bit we dont have asserts...
    return x + (y * config.outputWidth);
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
    if (global_id.x == 0) || (global_id.y == 0) || (global_id.x >= config.inputWidth - 1) || (global_id.y >= config.inputHeight - 1) {
        intermediateBuffer[coordsInput(global_id.x, global_id.y)] = pack2x16float(vec2f(0, 0));
        return;
    }

    // NOTE: there is a small advantage to splitting edge detection to collors instead of grayscaling, i think stevehanov just picked bad examples to show it off
    // source: https://stevehanov.ca/blog/?id=62

    let stepX = 1u + u32(floor((f32(config.inputWidth) / f32(config.outputWidth)) / 6.0)); // magic number for now
    let stepY = 1u + u32(floor((f32(config.inputHeight) / f32(config.outputHeight)) / 6.0)); // magic number for now
    // let stepX = 1u;
    // let stepY = 1u;

    let kernel = mat3x3f(
        brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x - stepX, global_id.y - stepY )] )),
        brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x - 0,     global_id.y - stepY )] )),
        brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x + stepX, global_id.y - stepY )] )),
        
        brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x - stepX, global_id.y + 0     )] )),
        0.0, // we could sample this, but for sobel it wont be used anyways
        brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x + stepX, global_id.y + 0     )] )),
        
        brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x - stepX, global_id.y + stepY )] )),
        brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x - 0,     global_id.y + stepY )] )),
        brightness(unpack4x8unorm( inputTexture[coordsInput(global_id.x + stepX, global_id.y + stepY )] )),
    );
    
    let sobelX = mat3x3f(
        -1, 0, 1,
        -2, 0, 2,
        -1, 0, 1,
    );
    let sobelY = mat3x3f(
        -1, -2, -1,
         0,  0,  0,
         1,  2,  1,
    );

    var gx: f32 = mdot(kernel, sobelX);
    var gy: f32 = mdot(kernel, sobelY);

    var magnitude: f32 = sqrt(gx * gx + gy * gy);
    var angle: f32 = atan2(gx, gy);

    var data = pack2x16float(vec2f(magnitude, angle));

    intermediateBuffer[coordsInput(global_id.x, global_id.y)] = data;
}

@compute
@workgroup_size(1)
fn do_scale(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let XL = global_id.x * config.inputWidth / (config.outputWidth + 1);
    let YL = global_id.y * config.inputHeight / (config.outputHeight + 1);
    let XR = ((global_id.x + 1) * config.inputWidth / (config.outputWidth + 1)) - 1;
    let YR = ((global_id.y + 1) * config.inputHeight / (config.outputHeight + 1)) - 1;
    let XC = (XL + XR) / 2;
    let YC = (YL + YR) / 2;

    let test = unpack2x16float(intermediateBuffer[coordsInput(XC, YC)]);
    let magnitude = test[0];
    let angle = test[1];

    let PI: f32 = 3.14159;
    
    let threshold: f32 = config.threshold;
    // var threshold: f32 = 1.0;
    var direction: u32 = 0u;

    // TODO improve by taking entire chunk

    if (magnitude > threshold) {
        if (angle < (2.0 * PI / 3.0) && angle > (PI / 3.0)) || (angle < (-2.0 * PI / 3.0) && angle > (-1.0 * PI / 3.0)) {
            direction = 4;
            // return "-";
        }
        if ((angle < PI / 6.0) && (angle > -1.0 * PI / 6.0)) || ((angle > 5.0 * PI / 6.0) || (angle < -5.0 * PI / 6.0)) {
            direction = 3;
            // return "|";
        }
        if ((angle > PI / 6.0) && (angle < PI / 3.0)) || ((angle > -5.0 * PI / 6.0) && (angle < -2.0 * PI / 3.0)) {
            direction = 1;
            // return "/";
        }
        if ((angle < -1.0 * PI / 6.0) && (angle > -1.0 * PI / 3.0)) || ((angle < 5.0 * PI / 6.0) && (angle > 2.0 * PI / 3.0)) {
            direction = 2;
            // return "\\";
        }
    }
    
    let packedColorPixel = inputTexture[coordsInput(XC, YC)];
    let colorPixel: vec4<f32> = unpack4x8unorm( packedColorPixel );

    let brightness = brightness(colorPixel); 

    outputBuffer[coordsOutput(global_id.x, global_id.y)] = PixelData(
        direction,
        packedColorPixel,
        brightness
    );
}
