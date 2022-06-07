// Implements a simple chromatic aberration effect.

#version 330 core

#shader_type vertex

uniform mat4 u_projectionView = mat4(1.0);

layout(location = 0)in vec2 a_position;
layout(location = 1)in vec2 a_uv;
layout(location = 2)in vec4 a_color;

out vec2 v_uv;

void main()
{
  v_uv = a_uv;
  
  gl_Position = vec4(a_position.x, - a_position.y, 0.0, 1.0) * u_projectionView;
}

#shader_type fragment

uniform sampler2D u_texture;
uniform float u_intensity;

in vec2 v_uv;

void main()
{
  gl_FragColor.r = texture(u_texture, vec2(v_uv.x + -1 * u_intensity, v_uv.y + -1 * u_intensity)).r;
  gl_FragColor.g = texture(u_texture, vec2(v_uv.x + 0 * u_intensity, v_uv.y + 0 * u_intensity)).g;
  gl_FragColor.b = texture(u_texture, vec2(v_uv.x + +1 * u_intensity, v_uv.y + +1 * u_intensity)).b;
  gl_FragColor.a = 1;
}
