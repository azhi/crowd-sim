#include <stdlib.h>
#include <unistd.h>
#include <stdio.h>
#include <time.h>
#include <sys/time.h>
#include <math.h>
#include <sys/select.h>

#include "controller.h"

void controller_error(const char* msg);
int wait_for_stdin();
unsigned char controller_read_byte();
unsigned short controller_read_short();
unsigned long controller_read_long();
char* controller_read_string();
double controller_read_double();

void controller_init_sdl(struct ControllerData* controller_data)
{
  struct SDLData sdl_data;
  sdl_init(&sdl_data, "Crowd Simulator: Animator");
  controller_data->sdl_data = sdl_data;
}

void controller_read_init_message(struct ControllerData* controller_data)
{
  controller_data->scene_file_name = controller_read_string();
  controller_data->scene_scale = controller_read_double();
}

void controller_load_textures(struct ControllerData* controller_data)
{
  sdl_set_svg_background(&controller_data->sdl_data, controller_data->scene_file_name);
  sdl_load_person_svg(&controller_data->sdl_data, controller_data->person_file_name, controller_data->person_file_scale, controller_data->scene_scale);
}

void controller_main_loop(struct ControllerData* controller_data)
{
  controller_data->current_simulation_time = 0.0;
  struct timeval current_time;
  gettimeofday(&current_time, NULL);
  double current_time_double = current_time.tv_sec + current_time.tv_usec / 1000000.0;
  controller_data->start_time = current_time_double;

  while (!feof(stdin)) {
    wait_for_stdin();
    double data_time = controller_read_double();

    sdl_clr(&controller_data->sdl_data);

    sdl_draw_texture(&controller_data->sdl_data, controller_data->sdl_data.background);

    unsigned char has_densities = controller_read_byte();
    if (has_densities) {
      long densities_count = controller_read_long();
      sdl_clear_density(&controller_data->sdl_data);
      for (long i = 0; i < densities_count; i++) {
          int x = controller_read_short();
          int y = controller_read_short();
          double density = controller_read_double();
          sdl_set_density(&controller_data->sdl_data, x, y, density);
        }
    }
    sdl_draw_density(&controller_data->sdl_data);

    long people_count = controller_read_long();
    for (long i = 0; i < people_count; i++) {
      short person_x = controller_read_short();
      short person_y = controller_read_short();
      double heading = controller_read_double();
      sdl_draw_person(&controller_data->sdl_data, person_x, person_y, heading);
    }

    gettimeofday(&current_time, NULL);
    current_time_double = current_time.tv_sec + current_time.tv_usec / 1000000.0;
    double delay = data_time - (current_time_double - controller_data->start_time);
    if (delay > 0.0) {
#ifdef DEBUG
      fprintf(stderr, "Ahead of time on %f s, waiting\n", delay);
#endif
      // we are ahead of data, wait
      struct timespec timespec;
      timespec.tv_sec = (time_t) floor(delay);
      timespec.tv_nsec = (long) floor(delay * 1000000000);
      nanosleep(&timespec, NULL);
    }
    sdl_update(&controller_data->sdl_data);
  }
}

void controller_shutdown(struct ControllerData* controller_data)
{
  sdl_shutdown(&controller_data->sdl_data);
  free(controller_data->scene_file_name);
}

char* controller_read_string()
{
  short string_length = controller_read_short();
  wait_for_stdin();
  char* buf = malloc(string_length + 1);
  int ret = fread(buf, 1, string_length, stdin);
  if (ret == 0)
    controller_error("Error while reading from stdin.");
  buf[string_length] = '\0';
  return buf;
}

unsigned char controller_read_byte()
{
  wait_for_stdin();
  unsigned char buf;
  int ret = fread(&buf, 1, 1, stdin);
  if (ret == 0)
    controller_error("Error while reading from stdin.");
  return buf;
}

unsigned short controller_read_short()
{
  wait_for_stdin();
  unsigned char buf[2] = {0, 0};
  int ret = fread(&buf, 2, 1, stdin);
  unsigned short res = ((buf[0] << 8) & 0xFF00) | (buf[1] & 0xFF);
  if (ret == 0)
    controller_error("Error while reading from stdin.");
  return res;
}

unsigned long controller_read_long()
{
  wait_for_stdin();
  unsigned char buf[4] = {0, 0, 0, 0};
  int ret = fread(&buf, 4, 1, stdin);
  unsigned long res = ((buf[0] << 24) & 0xFF000000) |((buf[1] << 16) & 0x00FF0000) |
                      ((buf[2] << 8) & 0x0000FF00) | (buf[3] & 0x000000FF);
  if (ret == 0)
    controller_error("Error while reading from stdin.");
  return res;
}

double controller_read_double()
{
  wait_for_stdin();
  double res = 0;
  int ret = fread(&res, 8, 1, stdin);
  if (ret == 0)
    controller_error("Error while reading from stdin.");
  return res;
}

int wait_for_stdin()
{
  fd_set read_set;
  FD_ZERO(&read_set);
  FD_SET(STDIN_FILENO, &read_set);
  int ret = select(STDIN_FILENO + 1, &read_set, NULL, NULL, NULL);
  return ret != -1 && FD_ISSET(STDIN_FILENO, &read_set);
}

void controller_error(const char* msg)
{
  fprintf(stderr, "[Controller] %s\n", msg);
  exit(1);
}
