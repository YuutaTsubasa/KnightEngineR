#version 300 es
precision highp float;

layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 texCoord;

out vec2 TexCoord;
out vec3 Color;

uniform mat4 model;

void main()
{
    gl_Position = model * vec4(position, 1.0);
    Color = color;
    TexCoord = texCoord;
}