#version 330 core
in vec3 fragPos;
in vec3 normalVect;
out vec4 FragColor;

uniform float u_time;
uniform vec2 u_resolution;
uniform vec3 u_lightpos;
uniform vec3 u_camera;

vec3 getLight(vec3 col) {

    vec3 lightCol = vec3(1.);

    vec3 newPos = u_lightpos;
    // newPos.x = 0.;
    // newPos.y = cos(u_time*0.2)*10.;
    // newPos.z = sin(u_time*0.2)*10.;

    // ambient lighting
    float ambientFactor = 0.;
    vec3 ambient = ambientFactor * lightCol;

    // diffuse lighting
    vec3 n = normalize(normalVect);
    vec3 lightRay = normalize(newPos - fragPos);
    float result = clamp(dot(lightRay, n), 0., 1.); //values equal to -1. behind the sphere so we need to put them btw 0. & 1.
    vec3 diffuse = result * lightCol;

    // specular lighting
    float specularFactor = .5;
    vec3 viewRay = normalize(u_camera - fragPos);
    //vec3 reflectDir = normalize(lightRay - 2*n*dot(lightRay, n));
    vec3 reflectDir = reflect(-lightRay, n);
    result = pow(max(dot(viewRay, reflectDir), 0.), 128);
    vec3 specular = specularFactor * result * lightCol;

    return col*(ambient + specular + diffuse);
}

void main()
{
   
    vec3 color = vec3(.8, .40, .65);
   
    FragColor = vec4(getLight(color), 1.0);
}