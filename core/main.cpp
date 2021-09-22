#include "funcs.hpp"
#include "utils.hpp"
#include "status.hpp"
#include "CLI11.hpp"

int main(int argc, char** argv) {

  CLI::App app{"Dragonfly Narrowband core functions"};

  app.require_subcommand(1);

  // ---------------
  auto sub_cooler = app.add_subcommand("cool", "Functions related to cooling and temperatures.");
  sub_cooler->require_subcommand(1);

  sub_cooler->add_subcommand("disable", "Turns off cooling.");
  sub_cooler->add_subcommand("get", "Get the current temperatures for various parts of the system.");

  auto sub_cooler_set = sub_cooler->add_subcommand("set", "Enables cooling and sets the target cooling temperature.");

  float target_temp;
  sub_cooler_set->add_option("temp", target_temp, "Target temperature in degrees C.")->required()->take_first();

  // ----------------

  auto sub_expose = app.add_subcommand("expose", "Take an exposure.");

  bool dark;
  sub_expose->add_option("--dark", dark, "Take a dark instead of a light frame.");
  
  float duration;
  sub_expose->add_option("--duration", duration, "Duration of exposure in seconds.")->required();

  int bin_x = 1;
  sub_expose->add_option("--binx", bin_x, "Amount of binning for the x axis. Defaults to 1.");

  int bin_y = 1;
  sub_expose->add_option("--biny", bin_y, "Amount of binning for the y axis. Defaults to 1.");
  // ------------------
  
  CLI11_PARSE(app, argc, argv);

	/* auto gateway = initialize_gateway(); */
  /* auto camera = initialize_camera(gateway).expect("Could not initialize camera!"); */
  /* auto sensor = initialize_sensor(camera).expect("Could not initialize sensor!"); */
  /* auto cooler = initialize_cooler(camera).expect("Could not initialize cooler!"); */

  /* auto sensor_info = get_sensor_info(sensor); */
  /* auto cooler_info = get_temp_info(camera, cooler); */

  /* ExposureInfo expinfo; */
  /* expinfo.bin_x = 1; */
  /* expinfo.bin_y = 1; */
  /* expinfo.duration = 0.2; */
  /* expinfo.light = true; */
  /* expinfo.readout_mode = ReadoutMode::High; */

  /* auto er = expose(camera, expinfo).expect("Could not expose!"); */

  /* std::cout << sensor_info.cooler_setpoint_min << " | " << sensor_info.cooler_setpoint_max << std::endl; */
  /* std::cout << sensor_info.exposure_duration_min << " | " << sensor_info.exposure_precision << std::endl; */

  /* std::cout << std::boolalpha << cooler_info.cooler_enabled << std::endl; */
  /* std::cout << cooler_info.sensor_temp << std::endl; */
  /* std::cout << cooler_info.cooler_setpoint << std::endl; */

	/* free_gateway(gateway); */
}
