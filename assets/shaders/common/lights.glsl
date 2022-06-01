#ifndef LIGHTS_H_
#define LIGHTS_H_

struct Light {
  vec3 position;
}

uniform Light u_lights[16];
uniform int   u_active_lights;

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