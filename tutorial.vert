#version 330 core
layout (location = 0) in vec3 position;
layout (location = 1) in vec3 color;
layout (location = 2) in vec2 texture;

out vec3 outColor;
out vec2 outTexture;

uniform mat4 transformation;

void main()
{
    gl_Position = transformation * vec4(position, 1.0f);
    outColor = color;
    outTexture = texture;
}
