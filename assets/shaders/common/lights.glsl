#ifndef SURREAL_LIGHTS_H_
#define SURREAL_LIGHTS_H_

// A light source in our 2d lighting pipeline.
struct Light {
  vec3 position; // world space position
  vec3 color; // ambient color
  float intensity; // normalised intensity
}

uniform Light u_lights[16]; // The active scene lights.
uniform int u_active_lights; // The number of active scene lights.

// Samples lighting information from the given world position.
vec3 sample_light(vec2 world_position) {
  vec3 color = vec3(0.0);
  
  for(int i = 0; i < u_active_lights; i ++ ) {
    Light light = u_lights[i];
    
    float distance = length(light.position - world_position);
    float attenuation = 1.0 / (distance * distance);
    
    color += attenuation;
  }
  
  return color;
}

#endif