#version 330 core

// Input vertex attributes
layout (location = 0) in vec3 aPosition;   // Vertex position
layout (location = 1) in vec3 aNormal;     // Vertex normal
layout (location = 2) in vec3 aColor;      // Vertex color

// Uniform matrices
uniform mat4 uModel;        // Model matrix
uniform mat4 uView;         // View matrix
uniform mat4 uProjection;   // Projection matrix
uniform mat3 uNormalMatrix; // Normal matrix (inverse transpose of model matrix)

// Output to fragment shader
out vec3 FragPos;    // Fragment position in world space
out vec3 Normal;     // Fragment normal in world space
out vec3 Color;      // Fragment color

void main()
{
    // Transform vertex position to world space
    vec4 worldPos = uModel * vec4(aPosition, 1.0);
    FragPos = worldPos.xyz;
    
    // Transform normal to world space
    Normal = normalize(uNormalMatrix * aNormal);
    
    // Pass color through
    Color = aColor;
    
    // Transform to clip space
    gl_Position = uProjection * uView * worldPos;
}
