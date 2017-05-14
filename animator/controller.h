#ifndef _CONTROLLER_H_
#define _CONTROLLER_H_

#include "sdl.h"

struct ControllerData {
  double current_simulation_time;
  double start_time;
  struct SDLData sdl_data;

  const char *person_file_name;
  double person_file_scale;
  char* scene_file_name;
  double scene_scale;
};

#define CURRENT_TIME_TYPE 0
#define LOCATIONS_TYPE 1
#define DENSITY_MAP_TYPE 2
#define STATISTICS_TYPE 3

void controller_init_sdl(struct ControllerData*);
void controller_read_init_message(struct ControllerData*);
void controller_load_textures(struct ControllerData*);
void controller_main_loop(struct ControllerData*);
void controller_shutdown(struct ControllerData*);

#endif
