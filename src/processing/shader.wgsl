@group(0)
@binding(0)
var<uniform> resolutions: vec4<u32>;

@group(0)
@binding(1)
var inputTexture: texture_2d<f32>;

@group(0)
@binding(2)
var<storage, read_write> intermediateBuffer: array<Test>;

@group(0)
@binding(3)
var<storage, read_write> outputBuffer: array<Test>;

struct Test {
    gx: f32,
    gy: f32,
    d1: f32,
    d2: f32,
}


fn coords(x: u32, y: u32) -> u32 {
    return x + (y * resolutions[0]);
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
    if (global_id.x >= resolutions[0] - 1) || (global_id.y >= resolutions[1] - 1) {
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
    intermediateBuffer[coords(global_id.x, global_id.y)] = Test(gx, gy, 0.0, 0.0);
}

@compute
@workgroup_size(1)
fn do_scale(@builtin(global_invocation_id) global_id: vec3<u32>) {
    outputBuffer[coords(global_id.x, global_id.y)] = intermediateBuffer[coords(global_id.x, global_id.y)];
}
