#include "sdl.h"

#include <math.h>
#include <cairo.h>
#include <librsvg/rsvg.h>

#define MAIN_FONT_FILE_PATH "/usr/share/fonts/TTF/DejaVuSerif.ttf"

SDL_Texture* sdl_load_svg(struct SDLData* sdl_data, const char* file, double scale, double angle);
void sdl_error(const char* msg);
void ttf_error(const char* msg);
void rsvg_error(const char* msg, GError* err);
void cairo_error(const char* msg);

void sdl_init(struct SDLData* sdl_data, const char* window_title)
{
  if (SDL_Init(SDL_INIT_EVERYTHING) != 0) {
    sdl_error("SDL_Init");
  }

  if (TTF_Init() != 0) {
    ttf_error("TTF_Init");
  }

  sdl_data->main_font = TTF_OpenFont(MAIN_FONT_FILE_PATH, 16);
  if (sdl_data->main_font == NULL)
    ttf_error("TTF_OpenFont");

  sdl_data->window = SDL_CreateWindow(window_title, 0, 0, SCREEN_WIDTH, SCREEN_HEIGHT,
    SDL_WINDOW_SHOWN);
  if (sdl_data->window == NULL)
    sdl_error("SDL_CreateWindow");

  sdl_data->renderer = SDL_CreateRenderer(sdl_data->window, -1, SDL_RENDERER_ACCELERATED | SDL_RENDERER_PRESENTVSYNC);
  if (sdl_data->renderer == NULL)
    sdl_error("SDL_CreateRenderer");

  SDL_SetRenderDrawColor(sdl_data->renderer, 255, 255, 255, 0);

  sdl_clr(sdl_data);
}

void sdl_set_svg_background(struct SDLData* sdl_data, const char* file)
{
  sdl_data->background = sdl_load_svg(sdl_data, file, 1, 0);

  SDL_Rect dst;
  SDL_QueryTexture(sdl_data->background, NULL, NULL, &dst.w, &dst.h);

  SDL_SetWindowSize(sdl_data->window, dst.w, dst.h);
  sdl_data->scene_width = dst.w;
  sdl_data->scene_height = dst.h;

  int btpp = 4;
  sdl_data->density_color_map = malloc(sdl_data->scene_width * sdl_data->scene_height * btpp);
  sdl_clear_density(sdl_data);
}

void sdl_load_person_svg(struct SDLData* sdl_data, const char* file, double person_svg_scale, double scene_scale)
{
  double scale = scene_scale / person_svg_scale;
  for (double angle = 0; angle < 360; angle++) {
    sdl_data->person_textures[(int) angle] = sdl_load_svg(sdl_data, file, scale, angle);
  }
}

void sdl_draw_texture(struct SDLData* sdl_data, SDL_Texture* texture)
{
  SDL_Rect dst;
  dst.x = 0;
  dst.y = 0;

  SDL_QueryTexture(texture, NULL, NULL, &dst.w, &dst.h);
  SDL_RenderCopy(sdl_data->renderer, texture, NULL, &dst);
}

void sdl_draw_person(struct SDLData* sdl_data, int x, int y, double heading)
{
  double angle = heading * 180 / M_PI + 90;
  long angle_int = lround(angle);
  while (angle_int >= 360) {
    angle_int -= 360;
  };
  while (angle_int < 0) {
    angle_int += 360;
  };
  if (angle_int > 359) {
    fprintf(stderr, "ALARM!! %f %d\n", angle, angle_int);
  }
  SDL_Texture* texture = sdl_data->person_textures[angle_int];

  SDL_Rect dst;
  SDL_QueryTexture(texture, NULL, NULL, &dst.w, &dst.h);
  dst.x = x - dst.w / 2;
  dst.y = y - dst.h / 2;

  SDL_RenderCopy(sdl_data->renderer, texture, NULL, &dst);
}

void sdl_set_density(struct SDLData* sdl_data, int x, int y, double density)
{
  density = fmin(density, sdl_data->density_map_max_threshold) - sdl_data->density_map_min_threshold;
  unsigned char white_channel = (unsigned char) 255 - floor(density * 255 / (sdl_data->density_map_max_threshold - sdl_data->density_map_min_threshold));
  sdl_data->density_color_map[y * sdl_data->scene_width + x] = 0xaaff0000 | (white_channel << 8) | white_channel;
}

void sdl_draw_density(struct SDLData* sdl_data)
{
  Uint32 rmask = 0x00ff0000;
  Uint32 gmask = 0x0000ff00;
  Uint32 bmask = 0x000000ff;
  Uint32 amask = 0xff000000;
  int bpp = 32;
  int btpp = 4;
  int stride = sdl_data->scene_width * btpp;

  SDL_Surface *sdl_surface = SDL_CreateRGBSurfaceFrom((void *) sdl_data->density_color_map, sdl_data->scene_width, sdl_data->scene_height,
    bpp, stride, rmask, gmask, bmask, amask);

  if (sdl_surface == NULL)
    sdl_error("SDL_CreateRGBSurfaceFrom");

  SDL_Texture *tex = SDL_CreateTextureFromSurface(sdl_data->renderer, sdl_surface);

  if (tex == NULL)
    sdl_error("SDL_CreateTextureFromSurface");

  SDL_FreeSurface(sdl_surface);

  sdl_draw_texture(sdl_data, tex);
  SDL_DestroyTexture(tex);
}

void sdl_clear_density(struct SDLData* sdl_data)
{
  for (int i = 0; i < sdl_data->scene_height; i++)
    for (int j = 0; j < sdl_data->scene_width; j++) {
      sdl_data->density_color_map[i * sdl_data->scene_width + j] = 0x00000000;
    }
}

SDL_Texture* sdl_get_statistics_texture(struct SDLData* sdl_data, char* statistics_text)
{
  Uint32 rmask = 0x00ff0000;
  Uint32 gmask = 0x0000ff00;
  Uint32 bmask = 0x000000ff;
  Uint32 amask = 0xff000000;
  int bpp = 32;
  int btpp = 4;
  int stride = sdl_data->scene_width * btpp;

  Uint32* faded_bg = malloc(sdl_data->scene_width * sdl_data->scene_height * btpp);
  for(int i = 0; i < sdl_data->scene_width * sdl_data->scene_height; i++) {
    faded_bg[i] = 0xdd999999;
  }

  SDL_Surface *sdl_surface = SDL_CreateRGBSurfaceFrom((void *) faded_bg, sdl_data->scene_width, sdl_data->scene_height,
    bpp, stride, rmask, gmask, bmask, amask);

  if (sdl_surface == NULL)
    sdl_error("SDL_CreateRGBSurfaceFrom");

  SDL_Color text_color = {0, 0, 0, 255};
  SDL_Surface* text_surface = TTF_RenderText_Blended_Wrapped(sdl_data->main_font, statistics_text, text_color, round(sdl_data->scene_width * 0.5));

  if (text_surface == NULL)
    ttf_error("TTF_RenderText_Blended");

  SDL_Rect dst = {sdl_data->scene_width / 2 - text_surface->w / 2, sdl_data->scene_height / 2 - text_surface->h / 2, text_surface->w, text_surface->h};
  int res = SDL_BlitSurface(text_surface, NULL, sdl_surface, &dst);
  if (res != 0) {
    sdl_error("SDL_BlitSurface");
  }

  SDL_Texture *tex = SDL_CreateTextureFromSurface(sdl_data->renderer, sdl_surface);

  if (tex == NULL)
    sdl_error("SDL_CreateTextureFromSurface");

  SDL_FreeSurface(sdl_surface);
  SDL_FreeSurface(text_surface);

  return tex;
}

unsigned char sdl_is_spacebar_pressed(struct SDLData* sdl_data)
{
  unsigned char spacebar_pressed = 0;
  SDL_Event event;
  while (SDL_PollEvent(&event)) {
    if (event.type == SDL_KEYUP) {
      if (event.key.keysym.sym == SDLK_SPACE) {
        spacebar_pressed = 1;
      }
    }
  }
  return spacebar_pressed;
}


void sdl_clr(struct SDLData* sdl_data)
{
  SDL_RenderClear(sdl_data->renderer);
}

void sdl_update(struct SDLData* sdl_data)
{
  SDL_RenderPresent(sdl_data->renderer);
}

void sdl_shutdown(struct SDLData* sdl_data)
{
  free(sdl_data->density_color_map);
  for (int i = 0; i < 360; i++) {
    SDL_DestroyTexture(sdl_data->person_textures[i]);
  }
  SDL_DestroyTexture(sdl_data->background);
  SDL_DestroyRenderer(sdl_data->renderer);
  SDL_DestroyWindow(sdl_data->window);
  TTF_CloseFont(sdl_data->main_font);
  SDL_Quit();
}

SDL_Texture* sdl_load_svg(struct SDLData* sdl_data, const char* file, double scale, double angle)
{
#ifdef DEBUG
  printf("load_svg(%s)\n", file);
#endif

  // Create an RSVG Handle
  GError *gerr = NULL;

  RsvgHandle *rsvg_handle = rsvg_handle_new_from_file(file, &gerr);
  if (rsvg_handle == NULL)
    rsvg_error("rsvg_handle_new_from_file", gerr);

  RsvgDimensionData dimensions;
  rsvg_handle_get_dimensions(rsvg_handle, &dimensions);
  int rwidth = dimensions.width;
  int rheight = dimensions.height;

#ifdef DEBUG
  fprintf(stderr, "SVG is %d x %d\n", rwidth, rheight);
#endif

  int width = ((float) rwidth * scale);
  int height = ((float) rheight * scale);

#ifdef DEBUG
  printf("scaling to %d x %d (%f scale)\n", width, height, scale);
#endif

  // scanline width
  int btpp = 4;
  int stride = width * btpp;
  unsigned char *image = calloc(stride * height, 1);

  cairo_surface_t *cairo_surf = cairo_image_surface_create_for_data(image,
    CAIRO_FORMAT_ARGB32,
    width, height, stride);

  if (cairo_surface_status(cairo_surf) != CAIRO_STATUS_SUCCESS)
    cairo_error("cairo_image_surface_create_for_data");

  cairo_t *cr = cairo_create(cairo_surf);
  if (cairo_status(cr) != CAIRO_STATUS_SUCCESS)
    cairo_error("cairo_create");

  cairo_translate(cr, width / 2.0, height / 2.0);
  cairo_rotate(cr, angle * M_PI / 180);
  cairo_translate(cr, - width / 2.0, - height / 2.0);
  cairo_scale(cr, scale, scale);
  cairo_rectangle(cr, 0.0, 0.0, width, height);
  cairo_set_source_rgba(cr, 1.0, 1.0, 1.0, 0.0);
  cairo_fill(cr);
  cairo_paint(cr);

  rsvg_handle_render_cairo(rsvg_handle, cr);

  cairo_surface_finish(cairo_surf);

  // Adjust the SDL surface to match the cairo surface created
  // (surface mask of ARGB)
  Uint32 rmask = 0x00ff0000;
  Uint32 gmask = 0x0000ff00;
  Uint32 bmask = 0x000000ff;
  Uint32 amask = 0xff000000;
  int bpp = 32;

  SDL_Surface *sdl_surface = SDL_CreateRGBSurfaceFrom((void *) image, width, height,
    bpp, stride, rmask, gmask, bmask, amask);

  if (sdl_surface == NULL)
    sdl_error("SDL_CreateRGBSurfaceFrom");

  SDL_Texture *tex = SDL_CreateTextureFromSurface(sdl_data->renderer, sdl_surface);

  if (tex == NULL)
    sdl_error("SDL_CreateTextureFromSurface");

  SDL_FreeSurface(sdl_surface);

  g_object_unref(rsvg_handle);
  cairo_surface_destroy(cairo_surf);
  free(image);
  cairo_destroy(cr);

  return tex;
}

void sdl_error(const char* msg)
{
  fprintf(stderr, "[SDL] Error: %s - %s\n", msg, SDL_GetError());
  exit(1);
}

void ttf_error(const char* msg)
{
  fprintf(stderr, "[SDL_TTF] Error: %s - %s\n", msg, TTF_GetError());
  exit(1);
}

void rsvg_error(const char* msg, GError* err)
{
  fprintf(stderr, "[RSVG] Error: %s failed\n", msg);
  exit(1);
}

void cairo_error(const char* msg)
{
  fprintf(stderr, "[Cairo] Error: %s failed\n", msg);
  exit(1);
}
