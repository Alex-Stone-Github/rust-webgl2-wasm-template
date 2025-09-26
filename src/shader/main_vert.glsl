#version 300 es

layout(location = 0) in vec3 vertexPosition;

void main(void) {
    gl_Position = vec4(vertexPosition, 1.0);
}