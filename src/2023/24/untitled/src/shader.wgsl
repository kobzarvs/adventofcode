#version 450

layout(set = 0, binding = 0) buffer Data {
    vec4 lines[];
};

layout(set = 0, binding = 1) buffer Out {
    vec4 colors[];
};

layout(location = 0) out vec4 fragColor;

void main() {
    fragColor = colors[gl_VertexIndex / 2];
}
