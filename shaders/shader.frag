#version 330 core

in vec3 vertexColor;
in vec2 texCoord;
out vec4 FragColor;

uniform sampler2D image;

void main() {
    //FragColor = vec4(vertexColor, 1.0);
    FragColor = (texture(image, texCoord) + vec4(vertexColor, 1.0)) / 2;
}