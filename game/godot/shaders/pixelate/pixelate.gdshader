// Pixel Shader by: Candorada
// https://www.shadertoy.com/view/Wcd3DS

shader_type canvas_item;
render_mode unshaded;

#include "./pixelateFont.gdshaderinc"

//mess around with how strong edge detection is
const float edgeThreshold = 0.04;

// how many pixels will 1 char take up, dont go bellow 8
const int pixelsPerCharacter = 16;

// mess with this to change the smoothing value of your colors.
// 1./2.2 for gamma correction
const float colorPower = 1.0/2.2;

uniform sampler2D screen_texture : hint_screen_texture, repeat_disable, filter_nearest;

vec4 getPixel(vec2 uv){
  return texture(screen_texture, uv);
}

vec2 soble(vec2 uv, vec2 screen_pixel_size){ // x == intensity and y == angle
  float iX = 0.0;
  float iY = 0.0;
  mat3 kernelX = mat3(
    vec3(-1.0, 0.0, 1.0),
    vec3(-2.0, 0.0, 2.0),
    vec3(-1.0, 0.0, 1.0)
  );
  mat3 kernelY = mat3(
    vec3(-1.0, -2.0, -1.0),
    vec3(0.0,  0.0,  0.0),
    vec3(1.0,  2.0,  1.0)
  );
  for(int y = -1;y<=1;y++){
    for(int x = -1;x<=1;x++){
      vec2 offset = vec2(float(x), float(y)) * screen_pixel_size;
      float intensity = dot(getPixel(uv + offset).rgb, vec3(0.299, 0.587, 0.114));
      //stole this inteisty code off chatGPT (since its acctually kinda smart)
      //if your wondering how this works then lemme explain
      //dot product takes all the vector components and multiplies them, then it adds them
      // (x,y,z) dot (x1,y1,z1) equals x*x1+y*y1+z*z1
      // since the effect that red has on your intensity is 0.299 this and green has a 0.587 effectiveness
      // and blue has a 0.114 effectiveness this dotproduct does the same thing as the grayscale equation 
      // that everyone uses, but its kina shorter and faster
      iX += kernelX[y+1][x+1] * intensity;
      iY += kernelY[y+1][x+1] * intensity;
    }
  }
  return vec2(iX,iY);
}

vec2 pixelateSimple(vec2 uv, vec2 screen_pixel_size){
  float px = float(pixelsPerCharacter);
  vec2 fC = floor((uv / screen_pixel_size) / px) * px;
  vec2 nUV = fC * screen_pixel_size;
  return nUV;
}

vec2 pixelateSample(vec2 uv, vec2 screen_pixel_size, out vec4 color){
  vec4 totalClr = vec4(0);
  vec2 totalSoble = vec2(0);
  float fx = float(pixelsPerCharacter);
  vec2 fC = floor(uv*(1.0/screen_pixel_size)/fx)*fx;
  for(float i=0.;i<fx*fx;i++){
    float x1 = mod(i,fx);
    float y1 = fx/i;
    vec2 uv = (fC+vec2(x1,y1))/(1.0/screen_pixel_size);
    totalClr +=getPixel(uv);
    totalSoble +=soble(uv, screen_pixel_size);
  }
  float a = totalClr.r*0.299+totalClr.g*0.587+totalClr.b*0.114;
  
  color = totalClr/fx/fx;
  return vec2(totalSoble/fx/fx);
  //return getPixel(fC/(1.0/screen_pixel_size).xy);   
}

vec4 toAscii(vec2 soble, vec4 color, vec2 fragCoord){
  float scale = 8.0 / float(pixelsPerCharacter);

  int x = int(mod(scale * fragCoord.x, 8.0));
  int y = int(mod(scale * fragCoord.y, 8.0));
  
  if (length(soble.xy) < edgeThreshold) {
    // Not a line, use ascii based on brightness
    float brightness = color.r * 0.299 + color.g * 0.587 + color. b * 0.114;
    brightness = clamp(brightness * 1.2, 0.0, 1.0);
    int i = int((brightness) * 9.0);

    vec3 asciiColor = color.rgb * float(getAscii(i, x, 8 - y));
    return vec4(asciiColor, 1.0);
  }

  float lineAngle = (atan(soble.y / soble.x) / (PI / 2.0) + 1.0) / 2.0;
  lineAngle = mod(1.0 - lineAngle + 0.5, 1.0);
  int i = int(lineAngle * 5.0);
  
  vec3 asciiColor = color.rgb * float(getAsciiLine(i, x, 8-y));
  return vec4(asciiColor, 1.0);
}

void fragment() {
  vec2 pixelUv = pixelateSimple(SCREEN_UV, SCREEN_PIXEL_SIZE);
  vec4 pixelColor = getPixel(pixelUv);

  //pixelColor = pow(pixelColor,vec4(colorPower));
  //float mix = 1.0 / max(pixelColor.r, max(pixelColor.g, pixelColor.b));

  vec2 soble = soble(pixelUv, SCREEN_PIXEL_SIZE);
  COLOR = toAscii(soble, 1.0 * pixelColor, FRAGCOORD.xy);
}
