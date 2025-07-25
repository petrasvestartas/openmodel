#version 330 core

// Input from vertex shader
in vec3 FragPos;    // Fragment position in world space
in vec3 Normal;     // Fragment normal in world space
in vec3 Color;      // Fragment color

// Uniform lighting parameters
uniform vec3 uLightPos;     // Light position in world space
uniform vec3 uLightColor;   // Light color
uniform vec3 uViewPos;      // Camera/view position in world space
uniform float uAmbientStrength;  // Ambient light strength
uniform float uSpecularStrength; // Specular light strength
uniform int uShininess;     // Specular shininess factor

// Output color
out vec4 FragColor;

void main()
{
    // Ambient lighting
    vec3 ambient = uAmbientStrength * uLightColor;
    
    // Diffuse lighting
    vec3 norm = normalize(Normal);
    vec3 lightDir = normalize(uLightPos - FragPos);
    float diff = max(dot(norm, lightDir), 0.0);
    vec3 diffuse = diff * uLightColor;
    
    // Specular lighting (Blinn-Phong)
    vec3 viewDir = normalize(uViewPos - FragPos);
    vec3 halfwayDir = normalize(lightDir + viewDir);
    float spec = pow(max(dot(norm, halfwayDir), 0.0), uShininess);
    vec3 specular = uSpecularStrength * spec * uLightColor;
    
    // Combine lighting with vertex color
    vec3 result = (ambient + diffuse + specular) * Color;
    
    FragColor = vec4(result, 1.0);
}
