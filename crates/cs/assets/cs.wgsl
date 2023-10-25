struct Particle {
  value : array<f32>,
};
// 存储缓冲区
@group(0) @binding(0) var<storage, read_write> particles: Particle;
//@group(0) @binding(1) var from_tex: texture_2d<f32>;
// 存储纹理
//@group(0) @binding(2) var to_tex: texture_storage_2d<rgba8unorm, write>;

@compute @workgroup_size(32, 32)
fn cs_main(@builtin(global_invocation_id) global_id: vec3<u32>) {
    let uv = vec2<i32>(global_id.xy);
    if(uv.x<20){
       particles.value[uv.x]=12.3;
    }
    // 读取存储缓冲区
   // let particle = particles[vu.x * uv.y];

    
}