#version 450

layout(location = 0) in vec2 v_position;
layout(set = 1, binding = 0 ) uniform WaterMaterial {
  vec4 u_color;
  vec3 u_size;
  vec2 u_resolution;
  float u_time;
};

layout(location = 0) out vec4 gl_color;

float draw_area(vec2 pt, vec2 size, vec2 center){
  vec2 half_size = size * 0.5;
  vec2 p = pt - center;
  float horz = step(-half_size.x, p.x) - step(half_size.x, p.x);
  float vert = step(-half_size.y, p.y) - step(half_size.y, p.y);
  return horz * vert;
}

float rand(vec2 st) {
  return fract(sin(dot(st, vec2(2234.1315, 1153.151)))* u_time * 0.5);
}

float noise(in vec2 st) {
  vec2 i = floor(st);
  vec2 f = fract(st);

  float a = rand(i);
  float b = rand(i + vec2(1.0, 0.0));
  float c = rand(i + vec2(0.0, 1.0));
  float d = rand(i + vec2(1.0, 1.0));

  vec2 u = f*f*(3.0-2.0*f);

  return mix(a, b, u.x) + (c - a) * u.y * (1.0 - u.x) + (d - b) * u.x * u.y;
}

void main(){
  vec2 st = gl_FragCoord.xy/u_resolution;
  vec2 pos = vec2(st*12.0);
  //vec4 color = u_color * draw_area(v_position.xy, vec2(0.42, 0.75), vec2(-0.01, -0.02));
  //vec4 color = u_color * draw_area(st, vec2((u_size.x * 2./u_resolution.x), (u_size.y * 2./u_resolution.y)), vec2(0.988,1.02));
  // Metal uses fom 0. to 2. as the size
  vec4 color = u_color * step(1.0 - (u_size.x/u_resolution.x), st.x) * step(st.x, 1.0 + (u_size.x/u_resolution.x)); 
  color *= step(1.0 - (u_size.y/u_resolution.y), st.y) * step(st.y, 1.0 + (u_size.y/u_resolution.y));
  float n = noise(pos);

  vec4 c = color * smoothstep(color.b, .3, noise(st*80.));
  gl_color = vec4(c.r, c.g, c.b, 0.0) + color;
}
