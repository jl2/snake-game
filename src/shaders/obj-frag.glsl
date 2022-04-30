#version 410 core

uniform vec3 Kd;
out vec4 color;

void main()
{
    color = vec4(Kd, 1.0);
}
