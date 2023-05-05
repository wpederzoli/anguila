#version 450
layout(location = 0) in vec2 position;

layout(location = 0) out vec2 v_position;

void main(){
  v_position = position.xy;
  gl_Position = vec4(v_position, 0.0, 1.0);
}
