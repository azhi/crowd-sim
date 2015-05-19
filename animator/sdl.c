#include "sdl.h"

#include <math.h>
#include <cairo.h>
#include <librsvg/rsvg.h>

SDL_Texture* sdl_load_svg(struct SDLData* sdl_data, const char* file, double scale, double angle);
void sdl_error(const char* msg);
void rsvg_error(const char* msg, GError* err);
void cairo_error(const char* msg);

void sdl_init(struct SDLData* sdl_data, const char* window_title)
{
  if (SDL_Init(SDL_INIT_EVERYTHING) != 0) {
    sdl_error("SDL_Init");
    exit(1);
  }

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
  if (angle < 0) {
    angle += 360;
  };
  SDL_Texture* texture = sdl_data->person_textures[(int) angle];

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

  /* cairo_surface_t *cairo_surf2 = cairo_image_surface_create(CAIRO_FORMAT_ARGB32, width, height); */
  /*  */
  /* if (cairo_surface_status(cairo_surf2) != CAIRO_STATUS_SUCCESS) */
  /*   cairo_error("cairo_image_surface_create"); */
  /*  */
  /* cairo_t *cr2 = cairo_create(cairo_surf2); */
  /* if (cairo_status(cr2) != CAIRO_STATUS_SUCCESS) */
  /*   cairo_error("cairo_create"); */


  /* cairo_set_source_surface(cr2, cairo_surf, 0, 0); */
  /*  */
  /* cairo_surface_finish(cairo_surf2); */

  char filename[1000];
  sprintf(filename, "/tmp/cairo%f.png", angle);
  cairo_surface_write_to_png(cairo_surf, filename);

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

  SDL_SetColorKey(sdl_surface, SDL_TRUE, SDL_MapRGB(sdl_surface->format, 192, 192, 192));

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
