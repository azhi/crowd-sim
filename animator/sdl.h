#ifndef _SDL_H_
#define _SDL_H_

#include <SDL.h>

#define SCREEN_WIDTH 1280
#define SCREEN_HEIGHT 720

struct SDLData {
  SDL_Window *window;
  SDL_Renderer *renderer;
  SDL_Texture* background;

  SDL_Texture* person_textures[360];

  Uint32* density_color_map;

  int scene_width;
  int scene_height;
};

void sdl_init(struct SDLData*, const char* window_title);
void sdl_set_svg_background(struct SDLData*, const char* file);
void sdl_load_person_svg(struct SDLData*, const char* file, double person_svg_scale, double scene_scale);
void sdl_draw_texture(struct SDLData*, SDL_Texture*);
void sdl_draw_person(struct SDLData*, int x, int y, double heading);
void sdl_set_density(struct SDLData*, int x, int y, double density);
void sdl_clear_density(struct SDLData*);
void sdl_draw_density(struct SDLData*);

void sdl_clr(struct SDLData*);
void sdl_update(struct SDLData*);
void sdl_shutdown(struct SDLData*);

#endif
