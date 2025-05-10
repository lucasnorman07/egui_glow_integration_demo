#version 330 core

layout (location = 0) in vec3 aPos;  // Position attribute
layout (location = 1) in vec2 aTexCoord;  // Texture coordinate attribute
layout (location = 2) in vec3 aColor; // Color attribute

out vec3 vertexColor; // Output color to the fragment shader
out vec2 texCoord;

uniform mat4 camMatrix;

void main() {
    texCoord = aTexCoord;
    // gl_Position = vec4(aPos.x - 0.2 * aPos.y, aPos.y, 0.0, 1.0); // Convert 2D to 4D position
    gl_Position = camMatrix * vec4(aPos, 1.0);
    vertexColor = aColor; // Pass color to fragment shader
}
