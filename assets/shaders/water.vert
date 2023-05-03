#version 450
<<<<<<< HEAD
<<<<<<< HEAD

layout(location = 0) in vec2 position;

layout(location = 0) out vec2 v_position;

void main(){
  v_position = position.xy;
  gl_Position = vec4(v_position, 0.0, 1.0);
=======
layout(location = 0) out vec3 v_position;
=======
>>>>>>> 5958d0b... Using uniforms

layout(location = 0) in vec2 position;

layout(location = 0) out vec2 v_position;

void main(){
<<<<<<< HEAD
  v_position = position;
  gl_Position = vec4(position, 1.0);
>>>>>>> 286cf33... GLSL setup
=======
  v_position = position.xy;
  gl_Position = vec4(v_position, 0.0, 1.0);
>>>>>>> 3f8637b... Shader water with hardcoded values
}
