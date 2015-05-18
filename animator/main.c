#include "controller.h"

int main(int argc, char **argv)
{
  struct ControllerData controller_data;
  controller_data.person_file_name = argv[1];
  controller_data.person_file_scale = atof(argv[2]);
  controller_init_sdl(&controller_data);
  controller_read_init_message(&controller_data);
  controller_load_textures(&controller_data);
  controller_main_loop(&controller_data);
  controller_shutdown(&controller_data);
  return 0;
}
