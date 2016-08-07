#version 330 core
in vec3 outColor;
in vec2 outTexture;

out vec4 color;
uniform sampler2D tex;

void main()
{
  color = texture(tex, outTexture);
}
